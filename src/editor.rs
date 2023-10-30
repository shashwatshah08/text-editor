use std::io::{stdin, stdout, Write};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

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
                    print!("\r\n");
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
