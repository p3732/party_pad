use super::super::stm;
use core;

use stm32f7::system_clock;

#[derive(Clone)]
pub struct xy {
    pub x_min: u16,
    pub x_max: u16,
    pub y_min: u16,
    pub y_max: u16,
}

impl stm {
    pub fn blink_led(&mut self) -> usize {
        // toggle the led
        let led_current = self.led.get();
        self.led.set(!led_current);
        system_clock::ticks()
    }

pub fn draw_fill_circle(stm: &mut stm,
                        x_center: u16,
                        y_center: u16,
                        radius: u16,
                        color: u16) {
    /*
    assert!(x_center + radius <= x_max && y_center + radius <= y_max);
    assert!(x_center - radius <= x_max && y_center - radius <= y_max);
    //assert!(is_legal_coord(x_center, y_center));
    */
        self.lcd.print_point_color_at(x_center, y_center, color);
        let mut x_offset = 0;
        for y_offset in 0..radius {
            x_offset = 0;
            while euclidean_dist_squared(x_center + x_offset,
                                         y_center + y_offset,
                                         x_center,
                                         y_center) < radius * radius {
                self.lcd
                    .print_point_color_at(x_center + x_offset, y_center + y_offset, color);
                self.lcd
                    .print_point_color_at(x_center + x_offset, y_center - y_offset, color);
                self.lcd
                    .print_point_color_at(x_center - x_offset, y_center + y_offset, color);
                self.lcd
                    .print_point_color_at(x_center - x_offset, y_center - y_offset, color);
                x_offset += 1;
            }
        }
        /*
    for x in x_low_bound..x_high_bound {
        for y in y_low_bound..y_high_bound {
            if euclidean_dist_squared(x, y, x_center, y_center) < radius * radius {
                stm.lcd.print_point_color_at(x, y, color);
            } else {
                stm.lcd.print_point_color_at(x, y, 0x8000);
            }
        }
    }*/
    }

    pub fn draw_rectangle(&mut self, xy: &xy, color: u16) {
        for x in xy.x_min..xy.x_max {
            self.lcd.print_point_color_at(x, xy.y_min, color);
            self.lcd.print_point_color_at(x, xy.y_max - 1, color);
        }
        for y in xy.y_min + 1..xy.y_max - 1 {
            self.lcd.print_point_color_at(xy.x_min, y, color);
            self.lcd.print_point_color_at(xy.x_max - 1, y, color);
        }
    }

    pub fn print_fill_rect(&mut self,
                           x_start: u16,
                           x_end: u16,
                           y_start: u16,
                           y_end: u16,
                           color: u16) {

        for x in x_start..x_end {
            for y in y_start..y_end {
                self.lcd.print_point_color_at(x as u16, y as u16, color);
            }
        }
    }

    pub fn draw_spiral(&mut self, xy: xy, color1: u16, color2: u16) {
        let mut yx = xy.clone();
        let mut start_color = color1;
        let mut color = start_color;

        while yx.y_min < 135 {
            // only works because 480 is dividable by 5

            for _ in 0..5 {
                self.draw_rectangle(&yx, color);
                // update variables
                yx.x_min += 1;
                yx.x_max -= 1;
                yx.y_min += 1;
                yx.y_max -= 1;
            }
            color = if color == color1 { color2 } else { color1 }
        }
        self.draw_rectangle(&yx, color);
    }


    pub fn print_bar_signed(&mut self, value: i16, pos: u16, width: u16, y_max: u16, color: u16) {
        /*
    let x_max = 480;
    let y_max: u16 = 272;

    assert!(pos < x_max);
    assert!(pos + width < x_max);
    */

        //TODO how to scale properly?
        let scale_factor = value as f32 * 10.0 / core::i16::MAX as f32;
        //let scale_factor = value as f32 / core::i16::MAX as f32;
        //TODO constants
        let value = core::cmp::max(core::cmp::min((y_max as f32 * scale_factor) as i16,
                                                  130 as i16),
                                   -130 as i16);
        //print_fill_rect(&mut lcd, pos, 20, pos+width, 20, 0x801F);

        if value > 0 {
            self.print_fill_rect(pos,
                                 pos + width,
                                 y_max / 2,
                                 (y_max as i16 / 2 + value) as u16,
                                 color);
        } else {
            self.print_fill_rect(pos,
                                 pos + width,
                                 (y_max as i16 / 2 + value) as u16,
                                 y_max / 2,
                                 color);

        }
    }
}

//TODO move to different file?
fn euclidean_dist_squared(x_1: u16, y_1: u16, x_2: u16, y_2: u16) -> u16 {
    let x_low;
    let x_high;
    let y_low;
    let y_high;
    if x_1 <= x_2 {
        x_low = x_1;
        x_high = x_2;
    } else {
        x_low = x_2;
        x_high = x_1;
    }
    if y_1 <= y_2 {
        y_low = y_1;
        y_high = y_2;
    } else {
        y_low = y_2;
        y_high = y_1;
    }
    x_high - x_low;
    y_high - y_low;
    (x_high - x_low) * (x_high - x_low) + (y_high - y_low) * (y_high - y_low)
}
