extern crate termion;
use termion::{color, style};

use std::io;


fn main() {
    println!("{}Red", color::Fg(color::Red));
    println!("{}{} yo !", termion::clear::All, termion::cursor::Goto(1, 1));
}


