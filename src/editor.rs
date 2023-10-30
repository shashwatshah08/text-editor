use std::io::{stdin, stdout, Write};

use termion::{event::Key, input::TermRead};

use crate::console::Console;

#[derive(Default)]
pub struct Editor {
    pub console: Console,
}

impl Editor {
    pub fn run(&mut self) {
        self.draw_rows();
        self.console.set_cursor(1, 1);

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

    fn draw_rows(&self) {
        for _ in 0..self.console.height - 1 {
            println!("~\r");
        }
    }
}
