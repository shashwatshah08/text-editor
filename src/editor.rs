use std::io::{stdin, stdout, Write};

use termion::{event::Key, input::TermRead};

use crate::console::Console;

#[derive(Default)]
pub struct Editor {
    pub console: Console,
}

impl Editor {
    pub fn run(&mut self) {
        self.console.render();

        for key in stdin().keys() {
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
                    self.console.process_character(c);
                }
                Key::Right | Key::Left | Key::Up | Key::Down => {
                    self.console.process_arrow_keys(key);
                }
                _ => {
                    print!("{:?}\r", key);
                    stdout().flush().unwrap();
                }
            }
        }
    }
}
