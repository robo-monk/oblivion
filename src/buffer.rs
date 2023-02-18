use std::fs;

#[derive(Debug)]
pub struct Buffer {
    filename: String,
    contents: String,
    modified: bool
}

impl Buffer {
    pub fn new(filename: String) -> Buffer {
        let contents = fs::read_to_string(filename.to_owned())
                            .expect("Unable to read file");

        Buffer {
            filename,
            contents,
            modified: false
        }
    }
}
