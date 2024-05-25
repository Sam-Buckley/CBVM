use crate::{
    builder::bytes,
    builder::bytes::*,
    byte,
    bytecode::{
        data::ByteData,
        ops::ArgType::*,
        ops::Operations::*,
        ops::*,
        types::Types::{self, *},
    },
    constant, emits,
    engine::memory::Heap,
    op, stream, typed,
};
use std::{fs::File, io::Read};



pub struct Reader {
    pos: usize,
    file: File,
    pub stream: Vec<u8>,
    pub bytes: ByteStream,
}

impl Reader {
    pub fn new(path: &str) -> Reader {
        Reader {
            pos: 0,
            file: File::open(path).unwrap(),
            stream: Vec::new(),
            bytes: ByteStream::new(),
        }
    }
    pub fn new_read(data: &Vec<u8>) -> Reader {
        Reader {
            pos: 0,
            file: File::open("temp").unwrap_or(File::create("temp").unwrap()),
            stream: data.clone(),
            bytes: ByteStream::new(),
        }
    }
    pub fn read(&mut self) -> () {
        let mut data = Vec::new();
        self.file.read_to_end(&mut data).unwrap();
        self.stream = data;
    }
    pub fn group(&mut self) -> ByteStream {
        while self.pos < self.stream.len() {
            self.handle(self.stream[self.pos]);
            self.pos += 1;
        }
        self.bytes.clone()
    }
    //function to handle a number, take the number of arguments from the contant in ops and add to bytestream
    fn handle(&mut self, n: u8) {
        let tp = Types::from(n);
        self.next();
        let arg = self.view() as u64;
        let byte = Byte{
            data: Box::new(arg),
            pos: 0,
            tp: tp,
        };
        self.bytes.emit(byte);
    }
    //functions to view current item in data, next, and increase counter of position
    fn view(&self) -> u8 {
        self.stream[self.pos]
    }
    fn next(&mut self) {
        if self.pos + 1 >= self.stream.len() {
            return;
        }
        self.pos += 1;
    }
    fn peek(&self) -> u8 {
        self.stream[self.pos + 1]
    }
}
