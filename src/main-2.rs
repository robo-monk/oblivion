extern crate termion;
use termion::{color, style, cursor, clear};
use termion::raw::IntoRawMode;

use std::io::{Write, stdout};


fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();



    write!(stdout, "{}{}Hey there!", color::Fg(color::Red), cursor::Goto(1,1)).unwrap();

    //println!("{}Red", color::Fg(color::Red));
    //println!("{}{} yo !", termion::clear::All, termion::cursor::Goto(1, 1));
}


