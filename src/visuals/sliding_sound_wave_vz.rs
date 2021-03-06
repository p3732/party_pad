use visuals::Visualizer;
use collections::boxed::Box;
use super::STM;
use stm32f7::lcd;
use visuals::constants::*;
use core;
use audio;


pub struct SlidingSoundVisualizer {
    buffer: [i16; X_MAX as usize],
    bar_width: u16,
}
impl Visualizer for SlidingSoundVisualizer {
    fn draw(&mut self, mut stm: &mut STM) {
        let mode = false;
        let mut mic_input: [i16; 1] = [0];
        audio::get_microphone_input(&mut stm, &mut mic_input, mode);
        for i in 1..((X_MAX / self.bar_width)) {
            //dont redraw whole vertical strip, only draw difference
            if self.buffer[i as usize] > self.buffer[(i - 1) as usize] {
                //new value is large than old now
                if self.buffer[i as usize] >= 0 && self.buffer[(i - 1) as usize] >= 0 {
                    //both positive => extend existing color
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 - self.buffer[i as usize]) as
                                              u16,
                                              ((Y_MAX / 2) as i16 -
                                               self.buffer[(i - 1) as usize]) as
                                              u16,
                                              RED);
                } else if self.buffer[i as usize] >= 0 && self.buffer[(i - 1) as usize] < 0 {
                    //new positive, old negative => draw new colored, draw old black
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 - self.buffer[i as usize]) as
                                              u16,
                                              Y_MAX / 2,
                                              RED);
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              Y_MAX / 2,
                                              ((Y_MAX / 2) as i16 -
                                               self.buffer[(i - 1) as usize]) as
                                              u16,
                                              BLACK);
                } else if self.buffer[i as usize] < 0 && self.buffer[(i - 1) as usize] < 0 {
                    //both negative => draw difference black
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 - self.buffer[i as usize]) as
                                              u16,
                                              ((Y_MAX / 2) as i16 -
                                               self.buffer[(i - 1) as usize]) as
                                              u16,
                                              BLACK);

                } else {
                    stm.lcd.set_background_color(lcd::Color::rgb(0, 0, 255));
                }
            } else {
                //new value is smaller than old one
                if self.buffer[i as usize] >= 0 && self.buffer[(i - 1) as usize] >= 0 {
                    //both positive => draw difference black
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 -
                                               self.buffer[(i - 1) as usize]) as
                                              u16,
                                              ((Y_MAX / 2) as i16 - self.buffer[i as usize]) as
                                              u16,
                                              BLACK);
                } else if self.buffer[i as usize] < 0 && self.buffer[(i - 1) as usize] >= 0 {
                    //new negative, old positive => extend existing color
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              Y_MAX / 2,
                                              ((Y_MAX / 2) as i16 - self.buffer[i as usize]) as
                                              u16,
                                              RED);
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 -
                                               self.buffer[(i - 1) as usize]) as
                                              u16,
                                              Y_MAX / 2,
                                              BLACK);
                } else if self.buffer[i as usize] < 0 && self.buffer[(i - 1) as usize] < 0 {
                    //both negative => draw difference colored
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 -
                                               self.buffer[(i - 1) as usize]) as
                                              u16,
                                              ((Y_MAX / 2) as i16 - self.buffer[i as usize]) as
                                              u16,
                                              RED);
                } else {
                    stm.lcd.set_background_color(lcd::Color::rgb(0, 255, 0));
                }
            }
            //shift values to the left
            self.buffer[i as usize - 1] = self.buffer[i as usize];
        }
        //TODO scaling?
        let scale_factor = mic_input[0] as f32 * 10.0 / core::i16::MAX as f32;
        let new_value = core::cmp::max(core::cmp::min((Y_MAX as f32 * scale_factor) as i16,
                                                      130 as i16),
                                       -130 as i16);
        self.buffer[((X_MAX / self.bar_width) - 1) as usize] = new_value;
        //TODO reprint like above? or write to buffer before?
        /*stm.print_bar_signed(mic_input[0],
                             (((X_MAX / self.bar_width) - 1) * self.bar_width),
                             self.bar_width,
                             Y_MAX,
                             RED);
        */
        /*
        if *self.current_pos + 2 * self.bar_width >= X_MAX {
            *self.current_pos = 0;
            stm.lcd.clear_screen();
        }
        stm.print_bar_signed(mic_input[0],
                             *self.current_pos,
                             self.bar_width,
                             Y_MAX,
                             RED);
        *self.current_pos += self.bar_width;
        */
    }
}
impl SlidingSoundVisualizer {
    pub fn new(bar_width: u16) -> Box<SlidingSoundVisualizer> {
        Box::new(SlidingSoundVisualizer {
                     bar_width: bar_width,
                     buffer: [0; X_MAX as usize],
                 })
    }
}
