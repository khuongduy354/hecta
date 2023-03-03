use std::io::{self, stdout, Write};

use termion::{event::Key, input::TermRead};

use crate::document::{self, Document, Position};
pub struct IDE {
    document: Document,
    should_quit: bool,
}
impl IDE {
    pub fn new() -> Self {
        let document = Document::new();
        let should_quit = false;
        Self {
            document,
            should_quit,
        }
    }
    pub fn take_key() -> Key {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key.unwrap();
            }
        }
    }
    pub fn process_input(&mut self) {
        let key = IDE::take_key();
        self.document.move_cursor(key);

        match key {
            Key::Ctrl('q') => self.should_quit = true,
            // Key::Char('\r') => self.text_area.text_buf.push('\n'),
            Key::Char(c) => {
                self.document.insert_char(c);
                // self.document.move_cursor(Key::Right);
            }
            _ => {}
        }
    }
    pub fn refresh_screen(&self) {
        // TextArea::cursor_hide();

        write!(stdout(), "{}", termion::cursor::Goto(1, 1));

        print!("{}", termion::clear::All,);
        self.document.draw_rows();

        let Position { x, y } = self.document.cursor_pos;
        write!(stdout(), "{}", termion::cursor::Goto(x as u16, y as u16));
        stdout().flush().unwrap();

        // TextArea::cursor_show();
    }
    pub fn run(&mut self) {
        loop {
            //quit
            if self.should_quit {
                break;
            }

            //refresh
            self.refresh_screen();

            //input
            self.process_input();
        }
    }
}
