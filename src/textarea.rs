const LINE_LIMIT: i16 = 250;
use std::io::{self, Write};
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

use std::{cmp, io::stdout};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::document::{self, Document};

pub struct TextArea {
    pub text_buf: String,
    pub cursor_pos: Position,
    _stdout: RawTerminal<io::Stdout>,
    pub document: document::Document,
}
impl TextArea {
    pub fn new() -> Self {
        let _stdout = stdout().into_raw_mode().unwrap();
        let text_buf = String::from("");
        let cursor_pos = Position { x: 1, y: 1 };
        let document = document::Document::new();
        Self {
            text_buf,
            cursor_pos,
            _stdout,
            document,
        }
    }
    pub fn insert_char(&mut self, c: char) {
        self.document.insert_char(c, &mut self.cursor_pos);
    }
    pub fn refresh_screen(&self) {
        // TextArea::cursor_hide();

        write!(stdout(), "{}", termion::cursor::Goto(1, 1));

        print!("{}", termion::clear::All,);
        self.document.draw_rows();

        write!(
            stdout(),
            "{}",
            termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
        );
        stdout().flush().unwrap();

        // TextArea::cursor_show();
    }
    pub fn move_cursor(&mut self, key: Key) {
        match key {
            Key::Up => self.cursor_pos.y -= 1,
            Key::Down => self.cursor_pos.y += 1,
            Key::Left => self.cursor_pos.x -= 1,
            Key::Right => self.cursor_pos.x += 1,
            _ => {}
        }
    }
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }
    pub fn process_input() -> Key {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key.unwrap();
            }
        }
    }
}
