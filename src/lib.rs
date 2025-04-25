#![no_std]

use num::Integer;
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::{serial_print, serial_println, vga_buffer::{
    is_drawable, plot, plot_num, plot_str, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH
}};

const WINDOW_HEIGHT : usize = BUFFER_HEIGHT / 2 - 1;
const WINDOW_WIDTH : usize = BUFFER_WIDTH / 2 - 1;
const START_ROW : usize = 1;
const HORIZONTAL_WINDOW_COUNT : usize = 2;

use core::{
    clone::Clone,
    cmp::{min, Eq, PartialEq},
    iter::Iterator,
    marker::Copy,
    prelude::rust_2024::derive,
};

#[derive(Copy, Clone, Eq, PartialEq)]

pub struct Window {
    letters : [char; WINDOW_HEIGHT * WINDOW_WIDTH],
    letter_count : usize,
    row : usize,
    col : usize,
    base_x : usize,
    base_y : usize,
}
pub struct SwimInterface {
    windows : [Window; 4],
    active_window : usize,
    window_height : usize,
    window_width : usize,
    col: usize,
    row: usize,
}

pub fn safe_add<const LIMIT: usize>(a: usize, b: usize) -> usize {
    (a + b).mod_floor(&LIMIT)
}

pub fn add1<const LIMIT: usize>(value: usize) -> usize {
    safe_add::<LIMIT>(value, 1)
}

pub fn sub1<const LIMIT: usize>(value: usize) -> usize {
    safe_add::<LIMIT>(value, LIMIT - 1)
}

impl Default for Window {
    fn default() -> Self {
        Self {
            letters: [' '; WINDOW_HEIGHT * WINDOW_WIDTH], 
            letter_count : 0,
            row : 0,
            col : 0,
            base_x: 0,
            base_y: 0,
        }
    }
}


impl Default for SwimInterface {
    fn default() -> Self {
        Self {
            windows : [Window::default(); 4],
            active_window : 0,
            window_height : WINDOW_HEIGHT,
            window_width : WINDOW_WIDTH,
            col: 0,
            row: 0,
        }
    }
}

impl SwimInterface {
    // fn letter_columns(&self) -> impl Iterator<Item = usize> + '_ {
    //     (0..self.num_letters).map(|n| safe_add::<BUFFER_WIDTH>(n, self.col))
    // }

    pub fn init(&mut self) {
        self.constr();
        self.update_window(0);
    }

    pub fn tick(&mut self) {
        // self.clear_current();
        // self.draw_current();
    }

    // fn clear_current(&self) {
    //     for x in self.letter_columns() {
    //         plot(' ', x, self.row, ColorCode::new(Color::Black, Color::Black));
    //     }
    // }

    // fn draw_current(&self) {
    //     for (i, x) in self.letter_columns().enumerate() {
    //         plot(
    //             self.letters[i],
    //             x,
    //             self.row,
    //             ColorCode::new(Color::Cyan, Color::Black),
    //         );
    //     }
    // }

    fn update_window(&mut self, new_id : usize) {
        let window_inf = self.windows[self.active_window];
        let change_pos = (window_inf.row * WINDOW_WIDTH) + window_inf.col;
        let old_char = self.windows[self.active_window].letters[change_pos];
        self.draw_border_inactive(self.windows[self.active_window], (self.active_window + 1) as isize);
        plot(old_char, self.windows[self.active_window].base_x + self.windows[self.active_window].col, self.windows[self.active_window].base_y + self.windows[self.active_window].row, ColorCode::new(Color::Black, Color::Black));
        self.active_window = new_id;
        let current_wind_info  = self.windows[new_id];
        self.draw_border_active(current_wind_info, (self.active_window + 1) as isize);
        self.col = current_wind_info.base_x + current_wind_info.col;
        self.row = current_wind_info.base_y + current_wind_info.row;
        plot(' ', self.col, self.row, ColorCode::new(Color::Green, Color::Green));
    }

    fn draw_border_active(&mut self, windinf : Window, wind_id : isize) {
        let mut h = 1;
        while h < self.window_height {
            plot('.', windinf.base_x - 1, h + windinf.base_y -1, ColorCode::new(Color::Green, Color::LightGreen));
            plot('.', self.window_width + windinf.base_x - 1, h + windinf.base_y - 2, ColorCode::new(Color::Green, Color::LightGreen));
            h += 1;
        }
        let mut i = 0;
        while i < ((self.window_width/2)-2) {
            plot('.', i + windinf.base_x - 1, windinf.base_y - 1, ColorCode::new(Color::Green, Color::LightGreen));
            plot('.', i + windinf.base_x - 1, self.window_height + windinf.base_y - 2, ColorCode::new(Color::Green, Color::LightGreen));
            i+=1
        }
        plot('F', i + windinf.base_x - 1, windinf.base_y - 1, ColorCode::new(Color::Green, Color::Black));
        plot('.', i + windinf.base_x - 1, self.window_height + windinf.base_y - 2, ColorCode::new(Color::Green, Color::LightGreen));
        i+=1;
        plot_num(wind_id, i + windinf.base_x - 1, windinf.base_y - 1, ColorCode::new(Color::Green, Color::Black));
        plot('.', i + windinf.base_x - 1, self.window_height + windinf.base_y - 2, ColorCode::new(Color::Green, Color::LightGreen));
        i+=1;
        while i < self.window_width+1 {
            plot('.', i + windinf.base_x - 1, windinf.base_y - 1, ColorCode::new(Color::Green, Color::LightGreen));
            plot('.', i + windinf.base_x - 1, self.window_height + windinf.base_y - 2, ColorCode::new(Color::Green, Color::LightGreen));
            
            i+=1
        }
    }
    fn draw_border_inactive(&mut self, windinf : Window, wind_id : isize) {
        let mut h = 1;
        while h < self.window_height {
            plot('.', windinf.base_x - 1, h + windinf.base_y -1, ColorCode::new(Color::Green, Color::Black));
            plot('.', self.window_width + windinf.base_x - 1, h + windinf.base_y - 2, ColorCode::new(Color::Green, Color::Black));
            h += 1;
        }
        let mut i = 0;
        while i < ((self.window_width/2)-2) {
            plot('.', i + windinf.base_x - 1, windinf.base_y - 1, ColorCode::new(Color::Green, Color::Black));
            plot('.', i + windinf.base_x - 1, self.window_height + windinf.base_y - 2, ColorCode::new(Color::Green, Color::Black));
            i+=1
        }
        plot('F', i + windinf.base_x - 1, windinf.base_y - 1, ColorCode::new(Color::Green, Color::Black));
        plot('.', i + windinf.base_x - 1, self.window_height + windinf.base_y - 2, ColorCode::new(Color::Green, Color::Black));
        i+=1;
        plot_num(wind_id, i + windinf.base_x - 1, windinf.base_y - 1, ColorCode::new(Color::Green, Color::Black));
        plot('.', i + windinf.base_x - 1, self.window_height + windinf.base_y - 2, ColorCode::new(Color::Green, Color::Black));
        i+=1;
        while i < self.window_width+1 {
            plot('.', i + windinf.base_x - 1, windinf.base_y - 1, ColorCode::new(Color::Green, Color::Black));
            plot('.', i + windinf.base_x - 1, self.window_height + windinf.base_y - 2, ColorCode::new(Color::Green, Color::Black));
            i+=1
        }
    }

    fn constr(&mut self) {
        plot_str("Swim Editor", 0, 0, ColorCode::new(Color::Green, Color::Black));
        let mut g = 1;
        let mut x = START_ROW;
        while g < 5 {
            //Drawing first line to Ind
            serial_print!("Doing a loop");
            let mut f = 0;
            while f < HORIZONTAL_WINDOW_COUNT {
                let mut h = 1;
                while h < self.window_height {
                    plot('.', f * (WINDOW_WIDTH), h + x, ColorCode::new(Color::Green, Color::Black));
                    h += 1;
                }
                let mut i = 0;
                while i < ((self.window_width/2)-2) {
                    plot('.', i + f * WINDOW_WIDTH, x, ColorCode::new(Color::Green, Color::Black));
                    i+=1
                }
                plot('F', i + f * WINDOW_WIDTH, x, ColorCode::new(Color::Green, Color::Black));
                i+=1;
                plot_num(g as isize, i + f * WINDOW_WIDTH, x, ColorCode::new(Color::Green, Color::Black));
                i+=1;
                while i < self.window_width {
                    plot('.', i + f * WINDOW_WIDTH, x, ColorCode::new(Color::Green, Color::Black));
                    i+=1
                }
                self.windows[&g-1].base_y = x + 1;
                self.windows[&g-1].base_x = f * WINDOW_WIDTH + 1;
                g+=1;
                f+=1;
            }
            let mut h = 1;
            while h < self.window_height {
                plot('.', HORIZONTAL_WINDOW_COUNT * (WINDOW_WIDTH), h + x, ColorCode::new(Color::Green, Color::Black));
                h += 1;
            }
            x += WINDOW_HEIGHT-1;
        }
        let mut i = 0;
        while i < WINDOW_WIDTH * 2 {
            plot('.', i, WINDOW_HEIGHT * 2 - 1, ColorCode::new(Color::Green, Color::Black));
            i+=1
        }
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c),
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        match key {
            KeyCode::F1 => {
                self.update_window(0);
            }
            KeyCode::F2 => {
                self.update_window(1);
            }
            KeyCode::F3 => {
                self.update_window(2);
            }
            KeyCode::F4 => {
                self.update_window(3);
            }
            KeyCode::ArrowDown => {
                self.windows[self.active_window].row += 1;
            }
            KeyCode::ArrowRight => {
                let window_inf = self.windows[self.active_window];
                let change_pos = (window_inf.row * WINDOW_WIDTH) + window_inf.col;
                let old_char = self.windows[self.active_window].letters[change_pos];
                plot(old_char, self.windows[self.active_window].base_x + self.windows[self.active_window].col,
                self.windows[self.active_window].base_y + self.windows[self.active_window].row,
                 ColorCode::new(Color::Black, Color::Black));
                self.windows[self.active_window].col += 1;
            }
            KeyCode::ArrowLeft => {
                self.windows[self.active_window].col -= 1;
            }
            KeyCode::ArrowUp => {
                self.windows[self.active_window].row -= 1;
            }
            KeyCode::Backspace => {

            }
            _ => {}
        }
    }

    fn handle_unicode(&mut self, key: char) {
        if is_drawable(key) {
            let window_inf = self.windows[self.active_window];
            serial_println!("{}", window_inf.row);
            serial_println!("{}", window_inf.col);
            let change_pos = (window_inf.row * WINDOW_WIDTH) + window_inf.col;
            serial_println!("{}", change_pos);
            self.windows[self.active_window].letters[change_pos] = key;
            plot(key, window_inf.base_x + window_inf.col, window_inf.base_y + window_inf.row, ColorCode::new(Color::Green, Color::Black));
            if window_inf.col < WINDOW_WIDTH - 2 {
                self.windows[self.active_window].col += 1;
            } else {
                self.windows[self.active_window].col = 0;
                self.windows[self.active_window].row += 1;
            }
            plot(' ', window_inf.base_x + self.windows[self.active_window].col, window_inf.base_y + self.windows[self.active_window].row, ColorCode::new(Color::Green, Color::Green));
        }
    }
}
