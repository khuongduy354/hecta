use std::{
    fmt::format,
    io::{stdout, Stdout, Write},
    ops::Index,
};

use termion::{
    event::Key,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

use crate::error::HectaError;

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
    pub fn get_row(&self, pos: usize) -> &String {
        self.rows.get(pos).unwrap()
    }
    // pub fn get_row_mut(&mut self, pos: usize) -> &mut String {
    //     self.rows.get_mut(pos).unwrap()
    // }
    // pub fn delete_char(key:Key) -> Result<(), HectaError> {
    //     match key{
    //         Key::Backspace=> ,
    //          Key::Delete=>,
    //     }
    //
    // }
    pub fn draw_rows(&self) {
        for row in &self.rows {
            println!("{}\r", row);
        }
    }

    pub fn move_cursor(&mut self, key: Key) {
        match key {
            Key::Up => {
                if self.cursor_pos.y == 1 {
                    return;
                }
                self.cursor_pos.y -= 1;
                if self.cursor_pos.x >= self.rows[self.cursor_pos.y - 1].len() {
                    self.cursor_pos.x = 1
                }
            }
            Key::Down => {
                if self.cursor_pos.y == self.rows.len() {
                    return;
                }
                self.cursor_pos.y += 1;

                if self.cursor_pos.x >= self.rows[self.cursor_pos.y - 1].len() {
                    self.cursor_pos.x = 1
                }
            }
            Key::Left => {
                if self.cursor_pos.x == 1 {
                    return;
                }
                self.cursor_pos.x -= 1;
            }
            Key::Right => {
                let row = self.get_row(self.cursor_pos.y - 1);
                if self.cursor_pos.x == self.width as usize {
                    return;
                }
                if self.cursor_pos.x > row.len() {
                    return;
                }
                self.cursor_pos.x += 1;
            }
            _ => {}
        }
    }
    pub fn insert_char(&mut self, c: char) {
        //enter
        if c == '\n' {
            // let current_row = self.get_row(self.cursor_pos.y - 1);
            // if self.cursor_pos.x >= current_row.len() {
            //     //new empty line
            //     self.rows.push(String::from(""));
            //     self.cursor_pos.y += 1;
            //     self.cursor_pos.x = 0;
            //     return;
            // }
            let y_idx = self.cursor_pos.y;
            if y_idx == self.rows.len() {
                self.rows.push(String::from(""));
                // self.cursor_pos.y += 1;
                // self.cursor_pos.x = 1;
            }

            //split remaining to newline
            let current_row = self.rows[y_idx - 1].clone();
            let splits = current_row.split_at(self.cursor_pos.x - 1);

            self.rows[y_idx - 1] = splits.0.to_string();
            self.rows.insert(y_idx, splits.1.to_string());

            self.cursor_pos.y += 1;
            self.cursor_pos.x = 1;
            return;
        }

        //add to left of cursor
        let row = &mut self.rows[self.cursor_pos.y - 1];
        row.insert(self.cursor_pos.x - 1, c);
        self.move_cursor(Key::Right);

        // row.push(index);

        // 1st char => add to new row
        // if self.rows.len() == 0 {
        //     self.rows.push(String::from(c));
        //     return;
        // }

        //get last row
        // let last_index = self.rows.len() - 1;
        // let last_row = self.rows.get(last_index);

        // end of line => new row
        // else add char to last row
        // if let Some(last_row) = last_row {
        //     if last_row.len() == (self.width) as usize {
        //         // print!("end of line");
        //         self.rows.push(String::from(c));
        //         cursor.y += 1;
        //     } else {
        //         self.rows.get_mut(last_index).unwrap().push(c);
        //     }
        // }
    }
}
