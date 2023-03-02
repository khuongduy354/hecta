const LINE_LIMIT: i16 = 250;
use std::io::{self, Write};
pub struct Position {
    x: usize,
    y: usize,
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
pub struct TextArea {
    pub text_buf: String,
    pub cursor_pos: Position,
    _stdout: RawTerminal<io::Stdout>,
}
impl TextArea {
    pub fn new() -> Self {
        let _stdout = stdout().into_raw_mode().unwrap();
        let text_buf = String::from("");
        let cursor_pos = Position { x: 1, y: 1 };
        Self {
            text_buf,
            cursor_pos,
            _stdout,
        }
    }
    pub fn draw_full(&self) {
        let text_buf = &self.text_buf;
        let offset = (LINE_LIMIT) as usize;
        let mut start = 0;
        let mut end = cmp::min(start + offset, text_buf.len());
        while end <= text_buf.len() {
            TextArea::draw_line(text_buf.get(start..end));
            start = end;
            end = cmp::min(start + offset, text_buf.len());

            //last iteration
            if end == text_buf.len() {
                TextArea::draw_line(text_buf.get(start..end));
                return;
            }
        }
    }
    pub fn draw_char(data: &str) {
        println!("{}\r", data);
    }
    pub fn draw_line(data: Option<&str>) {
        match data {
            Some(d) => println!("{}\r", d),
            _ => {}
        }
    }
    pub fn refresh_screen(&self) {
        TextArea::cursor_hide();
        TextArea::cursor_show();
        print!(
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
        );
        stdout().flush().unwrap();
        self.draw_full();
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
    pub fn cursor_to(pos: Position) {
        let Position { x, y } = pos;
        print!("{}", termion::cursor::Goto(x as u16, y as u16));
    }
}
