use crate::Renderer;
use std::fs;
use std::io::{stdin, stdout, Write};

// #[derive(Debug)]
pub struct Buffer {
    filename: String,
    contents: String,
    modified: bool,

    renderer: Renderer,
    index: u32

    // width: u16,
    // height: u16,
}

impl Buffer {
    pub fn new(filename: String, renderer: Renderer) -> Buffer {
        let contents = fs::read_to_string(filename.to_owned())
                        .expect("Unable to read file");

        Buffer {
            filename,
            contents,
            modified: false,
            renderer,
            index: 0
            // width,
            // height,
        }
    }

    pub fn append(&mut self, c: char) {
        // self.contents += c;
        self.contents.push(c);
        self.index += 1;
        if !self.modified {
            self.modified = true;
        }
    }
}
