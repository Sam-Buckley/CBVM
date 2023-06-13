extern crate alloc;
use alloc::vec::Vec;
use crate::bytecode::{
    ops::*,
    ops::ArgType::*,
    ops::Operations::*,
    types::*,
};

#[derive(Debug, Clone)]
pub struct ByteStream {
    pos: usize,
    bytes: Vec<Byte>
}

impl ByteStream {
    pub fn new() -> ByteStream {
        ByteStream {
            pos: 0,
            bytes: Vec::new()
        }
    }
    pub fn emit () {}
}

#[derive(Debug, Clone)]
pub struct Byte {
    data: u8,
    pos: usize
}