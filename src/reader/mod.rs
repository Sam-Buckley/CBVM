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

struct Reader {
    pos: usize,
    file: File,
    stream: Vec<u8>,
    bytes: ByteStream,
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
    pub fn read(&mut self) -> Result<Vec<u8>, ()> {
        //split the file by spaces and remove empty strings, read the entire file
        let mut data = Vec::new();
        self.file.read_to_end(&mut data).unwrap();
        let mut new = Vec::new();
        for i in data.split(|x| *x == b' ' || *x == b'\n') {
            if !i.is_empty() {
                new.push(i);
            }
        }
        //convert the strings to u8s
        let new: Vec<u8> = new
            .iter()
            .map(|x| {
                let mut num = 0;
                for i in *x {
                    num = num * 16 + u8::from_str_radix(&i.to_string(), 16).unwrap();
                }
                num
            })
            .collect();
        self.stream = new.clone();
        Ok(new)
    }
    fn group(&mut self, data: Vec<u8>) {
        let mut ptr = 0;
        while ptr < data.len() {
            self.handle(data[ptr]);
        }
    }
    fn read_byte(&mut self) -> Result<u8, ()> {
        if self.pos < self.stream.len() {
            let byte = self.stream[self.pos];
            self.pos += 1;
            Ok(byte)
        } else {
            Err(())
        }
    }
    fn handle(&mut self, item: u8) {
        match Operations::from(item) {
            FUNC => {
                self.bytes.emit(op!(FUNC));
                self.bytes.emitstream(stream!((TypeU64, 0x0)));
            }
            _ => todo!(),
        }
    }
}
