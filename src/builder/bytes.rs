
extern crate alloc;

use crate::bytecode::{
    data::ByteData,
    ops::ArgType::*,
    ops::Operations::*,
    types::Types::{self, *},
};
use crate::reader::Reader;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct ByteStream {
    pos: usize,
    pub bytes: Vec<Byte>,
}

impl From<Vec<ByteStream>> for ByteStream {
    fn from(streams: Vec<ByteStream>) -> Self {
        let mut stream = ByteStream::new();
        for s in streams {
            stream.emitstream(s);
        }
        stream
    }
}
impl From<ByteStream> for Vec<Byte> {
    fn from(stream: ByteStream) -> Self {
        stream.bytes
    }
}

impl From<Vec<u8>> for ByteStream {
    fn from(bytes: Vec<u8>) -> Self {
        let mut reader = Reader::new_read(&bytes);
        reader.group()
    }
}

impl From<ByteStream> for Vec<u8> {
    fn from(stream: ByteStream) -> Self {
        let mut bytes = Vec::new();
        for byte in stream.bytes {
            bytes.push(*byte.data as u8);
        }
        bytes
    }
}
impl From<Vec<Byte>> for ByteStream {
    fn from(bytes: Vec<Byte>) -> Self {
        ByteStream { pos: 0, bytes }
    }
}
impl From<&[Byte]> for ByteStream {
    fn from(bytes: &[Byte]) -> Self {
        ByteStream {
            pos: 0,
            bytes: bytes.to_vec(),
        }
    }
}
impl From<&[ByteStream]> for ByteStream {
    fn from(streams: &[ByteStream]) -> Self {
        let mut stream = ByteStream::new();
        for s in streams {
            stream.emitstream(s.clone());
        }
        stream
    }
}

impl Default for ByteStream {
    fn default() -> ByteStream {
        ByteStream {
            pos: usize::default(),
            bytes: Vec::default(),
        }
    }
}

impl ByteStream {
    #[allow(dead_code)]
    pub fn new() -> ByteStream {
        ByteStream {
            pos: 0,
            bytes: Vec::new(),
        }
    }
    pub fn emit(&mut self, byte: Byte) -> Self {
        self.bytes.push(byte);
        self.clone()
    }
    pub fn emitstream(&mut self, stream: ByteStream) -> Self {
        for byte in stream.bytes {
            self.bytes.push(byte);
        }
        self.clone()
    }

    pub fn stringify(&self) -> String {
        let mut string = String::new();
        //iterate over bytes, add type as u8 then data
        for byte in &self.bytes {
            string.push(byte.tp as u8 as char);
            string.push(char::from_u32(*byte.data as u32).unwrap_or('\0'));
        }
        string
    }
}

#[derive(Debug, Clone)]
pub struct Byte {
    pub data: Box<u64>,
    pub pos: usize,
    pub tp: Types,
}
impl Byte {
    pub fn unwrap(&self) -> u64 {
        *self.data.clone()
    }
}

//macro to take tuple of Type and Value and return a Byte
#[macro_export]
macro_rules! typed {
    ($tp:ident, $val:expr) => {
        Byte {
            data: Box::new($val),
            pos: 0,
            tp: Types::$tp,
        }
    };
}
#[macro_export]
macro_rules! op {
    ($op:ident) => {{
        use $crate::bytecode::ops::Operations::*;
        Byte {
            data: Box::new($op as u64),
            pos: 0,
            tp: Types::TypeOp,
        }
    }};
}
//macro to take a constant (u64) and return a Byte
#[macro_export]
macro_rules! constant {
    ($val:expr) => {
        Byte {
            data: Box::new($val),
            pos: 0,
            tp: Types::TypeU64,
        }
    };
}
#[macro_export]
macro_rules! string {
    //turn a string into a line of byte!(TypeU8, val)
    ($val:expr) => {
        {
            let mut stream = ByteStream::new();
            for c in $val.chars() {
                stream.emitstream(stream!((TypeU8, c as u64)));
            }
            stream
        }
    };
}
#[macro_export]
macro_rules! emits {
    ($($val:expr),*) => {
        {
            let mut stream = ByteStream::new();
            $(stream.bytes.push($val);)*
            stream
        }
    };
}

//macro to take a tuple and direct to the correct macro
#[macro_export]
macro_rules! byte {
    //either tuple of 2 (typed) or 1 (constant)
    (($tp:ident, $val:expr)) => {
        typed!($tp, $val)
    };
    ($val:expr) => {
        constant!($val)
    };
}
//macro to take a stream of tuples, and return a stream of Bytes
#[macro_export]
macro_rules! stream {
    //write the rule so the tuple can be either 1 or 2
    ($(($tp:ident, $val:expr)),*) => {
        {
            let mut stream = ByteStream::new();
            $(
                stream.bytes.push(byte!(($tp, $val)));
            )*
            stream
        }
    };
}
#[macro_export]
macro_rules! func {
    ($name:expr) => {
        Byte {
            data: Box::new(stringtohex($name.to_string())),
            pos: 0,
            tp: Types::TypeFunc,
        }
    };
}
pub fn stringtohex(string: String) -> u64 {
    let mut hex = String::new();
    for c in string.chars() {
        hex.push_str((format!("{:02x}", c as u8)).trim());
    }
    u64::from_str_radix(&hex, 16).unwrap()
}

impl Byte {
    fn stringify(&self) -> String {
        format!("{:02x}", *(self.data))
    }
    //function to format it like assembly, for every function put start of line, then args until next function type
    fn assembly(&self) -> String {
        let mut string = String::new();
        string.push_str(&format!("{:?}", self.tp));
        string.push_str(&format!("{:02x} ", *(self.data)));
        string
    }
}

//implement display for Byte, make it look like a programming language
impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02x}", *(self.data))
    }
}
//implement display for ByteStream, make it look like a programming language
impl std::fmt::Display for ByteStream {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::new();
        for byte in &self.bytes {
            //format is Type and Data, then a space
            string.push_str(&format!("{:x}{:02x} ", byte.tp as u8, *(byte.data)));
        }
        write!(f, "{}", string)
    }
}
