// mod buffer;
//mod mode;

// use Buffer;

use crate::{Buffer, RendererInterface};

// #[derive(Debug)]
pub struct Editor<'a> {
    buffers: Vec<Buffer<'a>>,
    active_buffer: Option<usize>
}

impl Editor<'_> {
    pub fn new() -> Self {
        Editor {
            buffers: Vec::new(), 
            active_buffer: None
        }
    }

    // pub fn add_buffer(&buffer)
}
