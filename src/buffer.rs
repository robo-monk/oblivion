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

    renderer: &'a mut dyn RendererInterface,
    index: u32, // width: u16,
                // height: u16,
}

impl Buffer<'_> {
    pub fn new(filename: String, renderer: &mut dyn RendererInterface) -> Buffer {
        let contents = fs::read_to_string(filename.to_owned()).expect("Unable to read file");
        // let test = Box::new(renderer);

        let mut buffer = Buffer {
            filename,
            contents,
            modified: false,
            renderer,
            index: 0, // width,
                      // height,
        };

        buffer.dumpf();
        buffer
    }

    // TEST stuff
    pub fn go(&mut self) {
        self.index -= 1;
        self.renderer.goto(self.index);
        self.renderer.flush();
    }

    pub fn dumpf(&mut self) {
        self.renderer.clear();
        self.renderer.write_string(&self.contents, 0);
    }
}

impl BufferInterface for Buffer<'_> {
    // fn new_line(&mut self) {
    // }

    fn append(&mut self, c: char) {
        // self.contents += c;
        self.contents.push(c);
        self.index += 1;
        if !self.modified {
            self.modified = true;
        }

        // println!("{c}");
        self.renderer.write(c, self.index);
        self.renderer.flush();
    }

    // fn getRenderer(&mut self) -> &dyn RendererInterface {
    //     return self.renderer;
    // }
}
