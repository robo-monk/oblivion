extern crate termion;
use std::io::{stdin, stdout, Stdout, Write};

use termion::cursor::BlinkingBlock;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor, style, terminal_size};

// #[derive(Debug)]
pub enum Renderer {
    Terminal(TermionRenderer),
}

// pub struct Renderer {}

pub trait RendererInterface {
    fn flush(&mut self);

    fn write(&mut self, c: char, index: u32);

    fn write_string(&mut self, s: String, index: u32);

    fn get_col_row_from_index(&self, index: u32) -> (u16, u16);
}

// #[derive(Debug)]
pub struct TermionRenderer {
    width: u16,
    height: u16,
    stdout: RawTerminal<Stdout>,
}

impl RendererInterface for TermionRenderer {
    fn get_col_row_from_index(&self, index: u32) -> (u16, u16) {
        let current_col = ((index / (self.width as u32)) + 1) as u16;
        let current_row = ((index % (self.width as u32)) + 1) as u16;
        (current_col, current_row)
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn write(&mut self, c: char, index: u32) {
        self.write_text(&format!("{}", c), index)
    }

    fn write_string(&mut self, s: String, index: u32) {
        self.write_text(&s, index)
    }
}

impl TermionRenderer {
    pub fn new((width, height): (u16, u16)) -> Self {
        let mut stdout = stdout().into_raw_mode().unwrap();

        // write!(
        //     stdout,
        //     "{}{}qq to exit. Type stuff, use alt, and so on...{}",
        //     clear::All,
        //     cursor::Goto(1, 1),
        //     cursor::BlinkingBlock
        // );
        let renderer = Self {
            width,
            height,
            stdout,
        };

        // renderer.write()

        renderer
    }

    pub fn write_text(&mut self, text: &str, index: u32) {
        let (current_col, current_row) = self.get_col_row_from_index(index);

        write!(
            self.stdout,
            "{}{}{}",
            cursor::Goto(current_row, current_col),
            text,
            cursor::BlinkingBlock
        )
        .expect("Could not write to the screen!");
    }

    // pub fn flush(&mut self) {
    //     self.stdout.flush().unwrap();
    // }

    // pub fn write(&mut self, c: char, index: u32) {
    //     let current_row = 1;
    //     let current_col = 1;

    //     write!(
    //         self.stdout,
    //         "{}{}{}",
    //         cursor::Goto(current_row, current_col),
    //         c,
    //         cursor::BlinkingBlock
    //     );
    // }
}
