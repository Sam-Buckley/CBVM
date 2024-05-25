extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use std::io::{Read, Write};

pub struct IO {
    pub in_buffer: Vec<u8>,
    pub out_buffer: Vec<u8>,
}
impl IO {
    pub fn write(&mut self, data: &[u8]) {
        self.out_buffer.extend_from_slice(data);
    }
    pub fn read(&mut self, size: usize) -> Vec<u8> {
        let mut data = Vec::new();
        for _ in 0..size {
            data.push(self.in_buffer.remove(0));
        }
        data
    }
    pub fn read_until(&mut self, delim: u8) -> Vec<u8> {
        let mut data = Vec::new();
        loop {
            let byte = self.in_buffer.remove(0);
            if byte == delim {
                break;
            }
            data.push(byte);
        }
        data
    }
    pub fn read_line(&mut self) -> String {
        let mut data = String::new();
        loop {
            let byte = self.in_buffer.remove(0);
            if byte == b'\n' {
                break;
            }
            data.push(byte as char);
        }
        data
    }
    pub fn flush(&mut self) {
        //write to stdout then clear
        std::io::stdout().write_all(&self.out_buffer).unwrap();
        self.out_buffer.clear();
    }
}

impl Default for IO {
    fn default() -> Self {
        Self {
            in_buffer: Vec::new(),
            out_buffer: Vec::new(),
        }
    }
}
