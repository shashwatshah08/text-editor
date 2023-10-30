use std::io::{stdout, Stdout, Write};

use termion::{
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

pub struct Console {
    pub height: u16,
    pub width: u16,
    pub stdout: RawTerminal<Stdout>,
    pub cursor_position: (u16, u16),
    pub max_line_num: u16,
}

impl Default for Console {
    fn default() -> Self {
        let (width, height) = terminal_size().unwrap();
        let stdout = stdout().into_raw_mode().unwrap();
        Self {
            height,
            width,
            stdout,
            cursor_position: (1, 1),
            max_line_num: 0,
        }
    }
}

impl Console {
    pub fn set_cursor(&mut self, x: u16, y: u16) {
        self.cursor_position = (x, y);
        write!(self.stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn process_backspace(&mut self) {
        if self.cursor_position.0 == 9 {
            if self.cursor_position.1 > 1 {
                self.set_cursor(9, self.cursor_position.1 - 1);
            }
        } else {
            print!("\x08 \x08");
            self.stdout.flush().unwrap();
            self.cursor_position = (self.cursor_position.0 - 1, self.cursor_position.1);
        }
    }

    pub fn process_enter(&mut self) {
        if self.cursor_position.1 > self.max_line_num {
            self.max_line_num = self.cursor_position.1;
            print!("\r\n{} | ", format_u16(self.cursor_position.1 + 1));
            self.stdout.flush().unwrap();
            self.set_cursor(9, self.cursor_position.1 + 1);
        } else {
            self.set_cursor(9, self.cursor_position.1 + 1);
        }
    }

    pub fn process_character(&mut self, c: char) {
        self.set_cursor(self.cursor_position.0 + 1, self.cursor_position.1);
        print!("{}", c);
        self.stdout.flush().unwrap();
    }
}

pub fn format_u16(n: u16) -> String {
    let s = n.to_string();
    let padding = " ".repeat(5 - s.len());
    format!("{}{}", padding, s)
}
