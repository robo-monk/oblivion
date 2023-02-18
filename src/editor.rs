// mod buffer;
//mod mode;

// use Buffer;

use crate::Buffer;

#[derive(Debug)]
pub struct Editor {
    buffers: Vec<Buffer>,
    active_buffer: Option<usize>
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            buffers: Vec::new(), 
            active_buffer: None
        }
    }

    // pub fn add_buffer(&buffer)
}
