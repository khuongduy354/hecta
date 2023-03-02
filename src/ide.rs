use termion::event::Key;

use crate::{document, textarea::*};
pub struct IDE {
    text_area: TextArea,
    should_quit: bool,
}
impl IDE {
    pub fn new() -> Self {
        let text_area = TextArea::new();
        let should_quit = false;
        Self {
            text_area,
            should_quit,
        }
    }
    pub fn process_input(&mut self) {
        let key = TextArea::process_input();
        self.text_area.move_cursor(key);

        match key {
            Key::Ctrl('q') => self.should_quit = true,
            // Key::Char('\r') => self.text_area.text_buf.push('\n'),
            Key::Char(c) => {
                self.text_area.insert_char(c);
                self.text_area.move_cursor(Key::Right);
            }
            _ => {}
        }
    }
    pub fn run(&mut self) {
        loop {
            //quit
            if self.should_quit {
                break;
            }

            //refresh
            self.text_area.refresh_screen();

            //input
            self.process_input();
        }
    }
}
//implement display for key
