use std::io::{stdin, stdout, Write};

use termion::{event::Key, input::TermRead};

use crate::console::Console;

#[derive(Default)]
pub struct Editor {
    pub console: Console,
    pub row_number: u16,
}

impl Editor {
    pub fn run(&mut self) {
        print!("\r{} | ", format_u16(self.row_number));
        stdout().flush().unwrap();

        for key in stdin().keys() {
            let key = key.unwrap();
            match key {
                Key::Ctrl('q') => {
                    println!("\r");
                    break;
                }
                Key::Backspace => {
                    print!("\x08 \x08");
                    stdout().flush().unwrap();
                }
                Key::Char('\n') => {
                    self.row_number += 1;
                    print!("\r\n{} | ", format_u16(self.row_number));
                    stdout().flush().unwrap();
                }
                Key::Char(c) => {
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
