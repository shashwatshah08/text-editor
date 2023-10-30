use std::io::{stdin, stdout, Write};

use termion::{event::Key, input::TermRead};

use crate::console::Console;

#[derive(Default)]
pub struct Editor {
    pub console: Console,
    pub max_line_num: u16,
}

impl Editor {
    pub fn run(&mut self) {
        print!("{}", termion::clear::All);
        stdout().flush().unwrap();
        self.console.set_cursor(1, 1);
        print!("\r{} | ", format_u16(self.console.cursor_position.1));
        stdout().flush().unwrap();
        self.console.set_cursor(9, 1);

        for key in stdin().keys() {
            let cursor_position = self.console.cursor_position;
            let key = key.unwrap();
            match key {
                Key::Ctrl('q') => {
                    println!("\r");
                    break;
                }
                Key::Backspace => {
                    if cursor_position.0 == 9 {
                        if cursor_position.1 > 1 {
                            self.console.set_cursor(9, cursor_position.1 - 1);
                        }
                    } else {
                        print!("\x08 \x08");
                        stdout().flush().unwrap();
                        self.console.cursor_position = (cursor_position.0 - 1, cursor_position.1);
                    }
                }
                Key::Char('\n') => {
                    if self.console.cursor_position.1 > self.max_line_num {
                        self.max_line_num = self.console.cursor_position.1;
                        print!("\r\n{} | ", format_u16(self.console.cursor_position.1 + 1));
                        stdout().flush().unwrap();
                        self.console.set_cursor(9, cursor_position.1 + 1);
                    } else {
                        self.console.set_cursor(9, cursor_position.1 + 1);
                    }
                }
                Key::Char(c) => {
                    self.console
                        .set_cursor(cursor_position.0 + 1, cursor_position.1);
                    print!("{}", c);
                    stdout().flush().unwrap();
                }
                _ => {
                    print!("{:?}\r", key);
                    stdout().flush().unwrap();
                }
            }
        }
    }
}

fn format_u16(n: u16) -> String {
    let s = n.to_string();
    let padding = " ".repeat(5 - s.len());
    format!("{}{}", padding, s)
}
