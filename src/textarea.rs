const LINE_LIMIT: i16 = 250;
use std::io::Write;
pub struct Position {
    x: usize,
    y: usize,
}

use std::{cmp, io::stdout};
pub struct TextArea {
    pub text_buf: String,
}
impl TextArea {
    pub fn new() -> Self {
        let text_buf = String::from("");
        Self { text_buf }
    }
    pub fn draw_full(&self) {
        let text_buf = &self.text_buf;
        let offset = (LINE_LIMIT - 1) as usize;
        let mut start = 0;
        let mut end = cmp::min(start + offset, text_buf.len() - 1);
        while end <= text_buf.len() - 1 {
            TextArea::draw_line(text_buf.get(start..end));
            start = end;
            end = cmp::min(start + offset, text_buf.len() - 1);

            //last iteration
            if end == text_buf.len() - 1 {
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
    pub fn refresh_screen() {
        print!("\x1b[2J");
        stdout().flush().unwrap();
    }
    pub fn update_cursor_pos(pos: &Position) {
        write!(
            stdout(),
            "{}x",
            termion::cursor::Goto(pos.x as u16, pos.y as u16)
        )
        .unwrap();
    }
}
