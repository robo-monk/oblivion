use crate::{Renderer, RendererInterface};
use std::fs;
use std::io::{stdin, stdout, Write};

pub trait BufferInterface {
    fn append(&mut self, c: char);
}

// #[derive(Debug)]
pub struct Buffer<'a> {
    filename: String,
    contents: String,
    modified: bool,

    renderer: &'a dyn RendererInterface,
    index: u32, // width: u16,
                // height: u16,
}

impl Buffer<'_> {
    pub fn new(filename: String, renderer: &dyn RendererInterface) -> Buffer {
        let contents = fs::read_to_string(filename.to_owned()).expect("Unable to read file");
        // let test = Box::new(renderer);

        Buffer {
            filename,
            contents,
            modified: false,
            renderer,
            index: 0, // width,
                      // height,
        }
    }
}

impl BufferInterface for Buffer<'_> {
    fn append(&mut self, c: char) {
        // self.contents += c;
        self.contents.push(c);
        self.index += 1;
        if !self.modified {
            self.modified = true;
        }
    }
}
