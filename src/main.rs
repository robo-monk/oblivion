extern crate termion;
use termion::{color, style, cursor, clear, terminal_size};

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
// use termion::raw::TermRead;
// TermRead

use std::io::{Write, stdout, stdin};


fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // write!(stdout, "{}{}Hey there!", color::Fg(color::Red), cursor::Goto(1,1)).unwrap();

    let (max_width, max_height) = terminal_size().unwrap();

    write!(stdout,
        "{}{}q to exit. Type stuff, use alt, and so on...{}",
        termion::clear::All,
        cursor::Goto(1, 1),
        cursor::Hide
    ).unwrap();

    stdout.flush().unwrap();

    let mut content = String::new();
    let mut current_row = 1;
    let mut current_col = 1;

    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => {

                current_row += 1;
                if current_row >= max_width {
                    current_row = 1;
                    current_col += 1;
                }

                write!(
                    stdout,
                    "{}{}",
                    cursor::Goto(current_row, current_col),
                    c
                );
                stdout.flush().unwrap();
            },
            _ => {}
        }

        // stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();

    //println!("{}Red", color::Fg(color::Red));
    //println!("{}{} yo !", termion::clear::All, termion::cursor::Goto(1, 1));
}


