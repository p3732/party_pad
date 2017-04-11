#![no_std]
#![no_main]
#![feature(collections)]

extern crate stm32f7_discovery as stm32f7;
extern crate collections;
// initialization routines for .data and .bss
extern crate r0;

mod hardware;
mod visuals;

use collections::boxed::Box;
use visuals::constants::*;
use visuals::default_visualizer::DefaultVisualizer;
use visuals::direct_mic_visualizer::DirectMicVisualizer;
use visuals::energy_visualizer::EnergyVisualizer;
use visuals::direct_mic_batch_vz::DirectMicBatchVisualizer;
use visuals::sliding_sound_wave_vz::SlidingSoundVisualizer;
use visuals::Visualizer;
use visuals::VizParameter;

use stm32f7::lcd;

#[inline(never)]
fn main() -> ! {
    let mut stm = hardware::STM::init();
    stm.lcd.clear_screen();
    //param struct for draw-method
    let mut param = VizParameter{spectrum: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                                   1.0, 1.0, 1.0, 1.0],
                                   mic_input: [1000;32]};

    /*
    DirectMicVZ shows the soundwave from one mic. Draws one sample at at time from left to right, followed by clearscreen
    ========================
    */
    let mut pos0 = 0; //TODO move completely to direct mic? lifetime issues..
    let direct_mic_viz: Box<Visualizer> = DirectMicVisualizer::new(&mut pos0, 2);
    /*
    DirectMicBatchVZ shows the soundwave from one mic like DirectSoundMic, but receives a batch of samples
    ========================
    */
    let mut pos1 = 0; 
    let direct_mic_batch_viz: Box<Visualizer> = DirectMicBatchVisualizer::new(&mut pos1, 2);
    /*
    SlidingSoundVZ shows the soundwave from one mic by sliding the shown area to the right upon receiving a new sample
    ========================
    */
    let mut pos2 = 0;
    let mut buffer = [0;X_MAX as usize];
    let sliding_viz: Box<Visualizer> = SlidingSoundVisualizer::new(&mut buffer, &mut pos2, 2);
    /*
    EnergyVZ shows a circle indicating the energy of the given samples
    ========================
    */
    let mut last_radius = 0;
    let energy_viz: Box<Visualizer> = EnergyVisualizer::new(&mut last_radius);
    /*
    The defult VZ draws something
     ========================
    */
    let default_viz: Box<Visualizer> =  DefaultVisualizer::new(
                          0xFFFF,
                          0xFC00);

    let mut current_visualizer = sliding_viz;
    let mut data0;
    let mut data1;
    let mut count;
    stm.lcd.set_background_color(lcd::Color::rgb(0, 0, 0));
    loop {
        count = 0;
        /*
        while count + 1 < param.mic_input.len() {
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            data0 = stm.sai_2.bdr.read().data();
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            data1 = stm.sai_2.bdr.read().data();

            param.mic_input[count] = data0 as i16;
            param.mic_input[count+1] = data1 as i16;

            count += 2;
        }
        */

        //while count < param.mic_input.len() {
        while count < 1 {
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            data0 = stm.sai_2.bdr.read().data();
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            data1 = stm.sai_2.bdr.read().data();

            param.mic_input[count] = data0 as i16;
            param.spectrum[count] = data0 as f32;

            count += 1;
        }

        current_visualizer.draw(&mut stm, &mut param);
        //        stm.lcd.clear_screen();

        /*
        stm.lcd.clear_screen();
        let radius = 0;
        stm.draw_fill_ring(240, 131, radius,radius + 20,cons::BLUE);
        */
        
    }
}

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static __DATA_END: u32;
        static mut __DATA_START: u32;
        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }
    let data_load = &__DATA_LOAD;
    let data_start = &mut __DATA_START;
    let data_end = &__DATA_END;
    let bss_start = &mut __BSS_START;
    let bss_end = &__BSS_END;

    // initializes the .data section
    //(copy the data segment initializers from flash to RAM)
    r0::init_data(data_start, data_end, data_load);
    // zeroes the .bss section
    r0::zero_bss(bss_start, bss_end);

    stm32f7::heap::init();

    // enable floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);

    main();
}