mod cli;
mod document;
mod error;
mod file_loader;
mod ide;
use cli::CliApp;
use file_loader::{load_file_to_doc, save_to_curr_dir};
use ide::IDE;
use std::io::{stdin, stdout, Write};

use termion::{event::Key, input::TermRead, raw::IntoRawMode, terminal_size};
fn mouse_cursor_ex() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::clear::All,).unwrap();
    print!("{}", termion::cursor::Goto(1, 1));
    print!("aaa");

    let mut x = 4;
    let mut y = 1;
    stdout.flush();
    loop {
        let key = stdin().keys().next().unwrap().unwrap();
        if key == Key::Ctrl('q') {
            break;
        }

        // write!(stdout, "{}a", termion::cursor::Goto(x, y)).unwrap();
        // stdout.flush();
        if key == Key::Right {
            x += 1;
        }
        if key == Key::Left {
            x -= 1;
        }
        write!(stdout, "{}", termion::cursor::Goto(x, y));
        stdout.flush();
    }
    // loop {}
}
fn term_size() {
    let size = terminal_size().unwrap();
    let width = size.0;
    println!("Terminal width: {}", width);
}

fn main() {
    let cliapp = CliApp::new();
    // load_file_to_doc(cliapp.file_path.unwrap().to_str().unwrap());
    // cliapp.args.path
    // mouse_cursor_ex();
    let mut ide = IDE::new();
    ide.run();
}
