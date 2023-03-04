use std::io::{stdout, Stdout};

use termion::{
    event::Key,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

use crate::error::HectaError;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
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

    pub fn get_row(&self, pos: usize) -> &String {
        self.rows.get(pos).unwrap()
    }
    pub fn draw_rows(&self) {
        for row in &self.rows {
            println!("{}\r", row);
        }
    }
    // event_handler
    pub fn delete(&mut self) {
        if self.rows[self.cursor_pos.y - 1].len() <= self.cursor_pos.x {
            return;
        }
        self.rows[self.cursor_pos.y - 1].remove(self.cursor_pos.x); //remove the right char of cursor
    }
    pub fn backspace(&mut self) {
        if self.cursor_pos.x == 1 {
            return;
        }
        self.rows[self.cursor_pos.y - 1].remove(self.cursor_pos.x - 2); //remove the left char of cursor
        self.cursor_pos.x -= 1;
    }
    pub fn insert_char(&mut self, c: char) {
        let cursor = self.cursor_pos;
        //enter
        if c == '\n' {
            if cursor.y == self.rows.len() {
                self.rows.push(String::from(""));
            }

            //split remaining to newline
            let current_row = self.get_row(cursor.y - 1).to_string();
            let splits = current_row.split_at(self.cursor_pos.x - 1);

            self.rows[cursor.y - 1] = splits.0.to_string();
            self.rows.insert(cursor.y, splits.1.to_string());

            self.set_cursor(Position::new(1, cursor.y + 1));
            return;
        }

        //add to left of cursor
        self.rows[cursor.y - 1].insert(cursor.x - 1, c);
        self.set_cursor(Position::new(cursor.x + 1, cursor.y));
    }

    // cursor  setter
    pub fn set_cursor(&mut self, mut pos: Position) -> Result<(), HectaError> {
        //limit moveable cursor position

        // 1 <= y <= document height
        if pos.y < 1 {
            pos.y = 1;
        }

        if pos.y > self.rows.len() {
            pos.y = self.rows.len();
        }

        // 1 <= x <= current_row width +1 <= document width
        if pos.x < 1 {
            pos.x = 1;
        }

        let current_row = self.get_row(pos.y - 1);
        if pos.x > current_row.len() + 1 {
            pos.x = current_row.len() + 1;
        }

        if pos.x > self.width as usize {
            pos.x = self.width as usize;
        }

        self.cursor_pos = pos;

        Ok(())
    }
    //cursor handler
    pub fn move_cursor(&mut self, key: Key) -> Result<(), HectaError> {
        let cursor = self.cursor_pos;

        match key {
            Key::Up => {
                self.set_cursor(Position::new(cursor.x, cursor.y - 1))?;
            }
            Key::Down => {
                self.set_cursor(Position::new(cursor.x, cursor.y + 1))?;
            }
            Key::Left => {
                self.set_cursor(Position::new(cursor.x - 1, cursor.y))?;
            }
            Key::Right => {
                self.set_cursor(Position::new(cursor.x + 1, cursor.y))?;
            }
            _ => {}
        }
        Ok(())
    }
}
