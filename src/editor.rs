use std::io::{stdin, stdout, Write};

use termion::{event::Key, input::TermRead};

use crate::console::{format_u16, Console};

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
                    self.console.process_backspace();
                }
                Key::Char('\n') => {
                    self.console.process_enter();
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
