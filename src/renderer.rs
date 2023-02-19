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

    fn clear(&mut self);
}

// #[derive(Debug)]
pub struct TermionRenderer {
    width: u16,
    height: u16,
    stdout: RawTerminal<Stdout>,
}

impl RendererInterface for TermionRenderer {
    fn get_col_row_from_index(&self, index: u32) -> (u16, u16) {
        let current_row = ((index / (self.width as u32)) + 1) as u16;
        let current_col = ((index % (self.width as u32))) as u16;
        (current_col, current_row)
    }


    fn clear(&mut self) {
        self.write_string(format!("{}", clear::All), 0);
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
        let stdout = stdout().into_raw_mode().unwrap();


        let mut renderer = Self {
            width,
            height,
            stdout,
        };

        renderer.clear();
        renderer.write_text("> Welcome to your oblivion Buffer. Press q to exit", 0);
        renderer.flush();

        renderer
    }

    pub fn write_text(&mut self, text: &str, index: u32) {
        let (current_row, current_col) = self.get_col_row_from_index(index);

        write!(
            self.stdout,
            "{}{}{}",
            cursor::Goto(current_row, current_col),
            text,
            cursor::BlinkingBlock
        )
        .expect("Could not write to the screen!");
    }
}
