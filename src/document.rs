use std::io::{stdout, Stdout, Write};

use termion::{
    event::Key,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Document {
    pub cursor_pos: Position,
    rows: Vec<String>,
    width: u16,
    _stdout: RawTerminal<Stdout>,
}

impl Document {
    pub fn new() -> Self {
        let _stdout = stdout().into_raw_mode().unwrap();
        let cursor_pos = Position { x: 1, y: 1 };
        let width = terminal_size().unwrap().0;
        let rows = vec![String::from("")];
        Self {
            width,
            rows,
            cursor_pos,
            _stdout,
        }
    }
    pub fn draw_rows(&self) {
        for row in &self.rows {
            println!("{}\r", row);
        }
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
    pub fn insert_char(&mut self, c: char) {
        let mut cursor = &mut self.cursor_pos;
        if c == '\n' {
            self.rows.push(String::from(""));
            cursor.y += 1;
            cursor.x = 0;
            return;
        }

        // 1st char => add to new row
        // if self.rows.len() == 0 {
        //     self.rows.push(String::from(c));
        //     return;
        // }

        //get last row
        let last_index = self.rows.len() - 1;
        let last_row = self.rows.get(last_index);

        // end of line => new row
        // else add char to last row
        if let Some(last_row) = last_row {
            if last_row.len() == (self.width) as usize {
                // print!("end of line");
                self.rows.push(String::from(c));
                cursor.y += 1;
            } else {
                self.rows.get_mut(last_index).unwrap().push(c);
            }
        }
    }
}
