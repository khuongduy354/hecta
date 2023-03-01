use std::io::stdin;

use termion::{
    event::{Event, Key},
    input::TermRead,
};

use crate::textarea::*;
pub struct IDE {
    text_area: TextArea,
}
impl IDE {
    pub fn new() -> Self {
        let text_area = TextArea::new();
        Self { text_area }
    }
    pub fn run(&mut self) {
        let stdin = stdin();
        for b in stdin.events() {
            let evt = b.unwrap();
            match evt {
                Event::Key(Key::Ctrl('q')) => break,
                Event::Key(Key::Char('\r')) => self.text_area.text_buf.push('\n'),
                Event::Key(Key::Char(c)) => {
                    self.text_area.text_buf.push(c);

                    //TODO: structure this into a better Buffer
                    TextArea::refresh_screen();
                    self.text_area.draw_full();
                }
                _ => {}
            }
        }
    }
}
