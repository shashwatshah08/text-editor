use std::io::{stdout, Stdout, Write};

use termion::{
    cursor::Goto,
    event::Key,
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
    pub fn render(&mut self) {
        print!("{}", termion::clear::All);
        self.cursor_position = (1, 1);
        write!(
            self.stdout,
            "{}",
            Goto(self.cursor_position.0, self.cursor_position.1)
        )
        .unwrap();
        print!("\r{} | ", format_u16(self.cursor_position.1));
        self.cursor_position = (9, 1);
        write!(
            self.stdout,
            "{}",
            Goto(self.cursor_position.0, self.cursor_position.1)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn process_backspace(&mut self) {
        if self.cursor_position.0 == 9 {
            if self.cursor_position.1 > 1 {
                self.cursor_position = (9, self.cursor_position.1 - 1);
                write!(
                    self.stdout,
                    "{}",
                    Goto(self.cursor_position.0, self.cursor_position.1)
                )
                .unwrap();
                self.stdout.flush().unwrap();
            }
        } else {
            self.cursor_position = (self.cursor_position.0 - 1, self.cursor_position.1);
            print!("\x08 \x08");
            self.stdout.flush().unwrap();
        }
    }

    pub fn process_enter(&mut self) {
        if self.cursor_position.1 > self.max_line_num {
            self.max_line_num = self.cursor_position.1;
            print!("\r\n{} | ", format_u16(self.cursor_position.1 + 1));
            self.cursor_position = (9, self.cursor_position.1 + 1);
            write!(
                self.stdout,
                "{}",
                Goto(self.cursor_position.0, self.cursor_position.1)
            )
            .unwrap();
        } else {
            self.cursor_position = (9, self.cursor_position.1 + 1);
            write!(self.stdout, "{}", Goto(9, self.cursor_position.1)).unwrap();
        }
        self.stdout.flush().unwrap();
    }

    pub fn process_character(&mut self, c: char) {
        print!("{}", c);
        self.cursor_position = (self.cursor_position.0 + 1, self.cursor_position.1);
        write!(
            self.stdout,
            "{}",
            Goto(self.cursor_position.0, self.cursor_position.1)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn process_arrow_keys(&mut self, key: Key) {
        match key {
            Key::Left => {
                if self.cursor_position.0 > 9 {
                    self.cursor_position = (self.cursor_position.0 - 1, self.cursor_position.1);
                    write!(
                        self.stdout,
                        "{}",
                        Goto(self.cursor_position.0, self.cursor_position.1)
                    )
                    .unwrap();
                    self.stdout.flush().unwrap();
                } else if self.cursor_position.1 > 1 {
                    self.cursor_position = (9, self.cursor_position.1 - 1);
                    write!(
                        self.stdout,
                        "{}",
                        Goto(self.cursor_position.0, self.cursor_position.1)
                    )
                    .unwrap();
                    self.stdout.flush().unwrap();
                }
            }
            Key::Right => {
                if self.cursor_position.0 < self.width {
                    self.cursor_position = (self.cursor_position.0 + 1, self.cursor_position.1);
                    write!(
                        self.stdout,
                        "{}",
                        Goto(self.cursor_position.0, self.cursor_position.1)
                    )
                    .unwrap();
                } else {
                    self.cursor_position = (9, self.cursor_position.1 + 1);
                    write!(
                        self.stdout,
                        "{}",
                        Goto(self.cursor_position.0, self.cursor_position.1)
                    )
                    .unwrap();
                }
                self.stdout.flush().unwrap();
            }
            Key::Up => {
                if self.cursor_position.1 > 1 {
                    self.cursor_position = (self.cursor_position.0, self.cursor_position.1 - 1);
                    write!(
                        self.stdout,
                        "{}",
                        Goto(self.cursor_position.0, self.cursor_position.1)
                    )
                    .unwrap();
                    self.stdout.flush().unwrap();
                }
            }
            Key::Down => {
                if self.cursor_position.1 <= self.max_line_num {
                    self.cursor_position = (self.cursor_position.0, self.cursor_position.1 + 1);
                    write!(
                        self.stdout,
                        "{}",
                        Goto(self.cursor_position.0, self.cursor_position.1)
                    )
                    .unwrap();
                    self.stdout.flush().unwrap();
                }
            }
            _ => {}
        }
    }
}

pub fn format_u16(n: u16) -> String {
    let s = n.to_string();
    let padding = " ".repeat(5 - s.len());
    format!("{}{}", padding, s)
}
