use crate::input::bindings::KeyCode;
use std::collections::HashSet;

pub struct KeyBuffer {
    buffer: HashSet<usize>,
}

impl KeyBuffer {
    pub fn new() -> Self {
        Self {
            buffer: HashSet::default(),
        }
    }

    pub fn is_pressed(&self, key_code: KeyCode) -> bool {
        self.is_buffered(key_code.mask.into()) && self.is_buffered(key_code.code.into())
    }

    pub fn is_buffered(&self, key: usize) -> bool {
        println!("trying: {} contents: {:?}", key, self.buffer);

        self.buffer.iter().find(|x| x == &&key).is_some()
    }

    pub fn add_to_buffer(&mut self, key: usize) {
        self.buffer.insert(key);
    }

    pub fn remove_from_buffer(&mut self, key: usize) {
        self.buffer.retain(|x| x != &key);
    }
}
