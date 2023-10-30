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
        }
    }
}

impl Console {
    pub fn set_cursor(&mut self, x: u16, y: u16) {
        self.cursor_position = (x, y);
        write!(self.stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
        self.stdout.flush().unwrap();
    }
}
