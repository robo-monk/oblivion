extern crate termion;
use termion::{color, style, cursor, clear};

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

    write!(stdout,
        "{}{}q to exit. Type stuff, use alt, and so on...{}",
        termion::clear::All,
        cursor::Goto(1, 1),
        cursor::Hide
    ).unwrap();

    stdout.flush().unwrap();

    for k in stdin.keys() {
        write!(stdout, "{}{}", cursor::Goto(1, 1), clear::CurrentLine).unwrap();

        match k.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => {
                write!(stdout, "{}", c);
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


