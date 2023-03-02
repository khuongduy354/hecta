use std::io::{stdout, Write};

use termion::terminal_size;

use crate::textarea::Position;

pub struct Document {
    rows: Vec<String>,
    width: u16,
}

impl Document {
    pub fn new() -> Self {
        let width = terminal_size().unwrap().0;
        let rows = vec![String::from("")];
        Self { width, rows }
    }
    pub fn draw_rows(&self) {
        for row in &self.rows {
            println!("{}\r", row);
        }
    }
    pub fn insert_char(&mut self, c: char, cursor: &mut Position) {
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

//editing text
//char push
// eol new row( cursor position )
// enter new row
