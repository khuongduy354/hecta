use std::io::{self, stdout, Write};

use termion::{event::Key, input::TermRead};

use crate::{
    document::{Document, Position},
    error::HectaError,
};

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
    pub fn process_input(&mut self) -> Result<(), HectaError> {
        let key = IDE::take_key();
        self.document.move_cursor(key);

        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Delete => {
                self.document.delete();
            }
            Key::Backspace => {
                self.document.backspace();
            }
            Key::Char(c) => {
                self.document.insert_char(c);
            }

            _ => {}
        }
        Ok(())
    }
    pub fn refresh_screen(&self) -> Result<(), HectaError> {
        write!(stdout(), "{}", termion::cursor::Goto(1, 1)).unwrap();

        print!("{}", termion::clear::All,);
        self.document.draw_rows();

        let Position { x, y } = self.document.cursor_pos;
        write!(stdout(), "{}", termion::cursor::Goto(x as u16, y as u16)).unwrap();

        stdout().flush().unwrap();
        Ok(())
    }
    pub fn run(&mut self) -> Result<(), HectaError> {
        loop {
            //quit
            if self.should_quit {
                return Ok(());
            }

            //refresh
            self.refresh_screen()?;

            //input
            self.process_input()?;
        }
    }
}
