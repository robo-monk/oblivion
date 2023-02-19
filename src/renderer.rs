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

    fn write(&mut self, c: char);
}

// #[derive(Debug)]
pub struct TermionRenderer {
    width: u16,
    height: u16,
    stdout: RawTerminal<Stdout>,
}

impl RendererInterface for TermionRenderer {
    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn write(&mut self, c: char) {
        let current_row = 1;
        let current_col = 1;

        write!(
            self.stdout,
            "{}{}{}",
            cursor::Goto(current_row, current_col),
            c,
            cursor::BlinkingBlock
        );
    }
}

impl TermionRenderer {
    pub fn new((width, height): (u16, u16)) -> Self {
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(
            stdout,
            "{}{}q to exit. Type stuff, use alt, and so on...{}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::BlinkingBlock
        );
        Self {
            width,
            height,
            stdout,
        }
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn write(&mut self, c: char) {
        let current_row = 1;
        let current_col = 1;

        write!(
            self.stdout,
            "{}{}{}",
            cursor::Goto(current_row, current_col),
            c,
            cursor::BlinkingBlock
        );
    }
}
