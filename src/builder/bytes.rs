#![allow(dead_code, non_snake_case, unused_imports, unused_macros, unused_variables, unused_mut, unused_parens, unused_assignments, unused_braces, unused_import_braces)]
extern crate alloc;

use alloc::vec::Vec;
use crate::bytecode::{
    ops::ArgType::*,
    ops::Operations::*,
    types::Types::{self, *},
    data::ByteData
};

#[derive(Debug, Clone)]
pub struct ByteStream {
    pos: usize,
    pub bytes: Vec<Byte>
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
        ByteStream {
            pos: 0,
            bytes
        }
    }
}
impl From<&[Byte]> for ByteStream {
    fn from(bytes: &[Byte]) -> Self {
        ByteStream {
            pos: 0,
            bytes: bytes.to_vec()
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

impl ByteStream {
    #[allow(dead_code)]
    pub fn new() -> ByteStream {
        ByteStream {
            pos: 0,
            bytes: Vec::new()
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
            //add as hex
            string.push_str(&(format!("{:02x}", byte.tp as u8)).trim());
            string.push(' ');
            //add as hex
            let data = byte.clone().data.clone();
            //filter any null bytes
            string.push_str(&(match byte.tp {
                //format as hex, numbers should be 2 digits+ if not add a 0
                //if the number is larger than 2 digits, remove any whitespace
                TypeU8 => format!("{:02x}", *data),
                TypeU64 => format!("{:016x}", *data),
                TypeI8 => format!("{:02x}", *data),
                TypeI64 => format!("{:016x}", *data),
                TypeF32 => format!("{:08x}", *data),
                TypeF64 => format!("{:016x}", *data),
                TypeU128 => format!("{:032x}", *data),
                TypeI128 => format!("{:032x}", *data),
                TypeAddr => format!("{:016x}", *data),
                TypeReg => format!("{:016x}", *data),
                TypeFunc => format!("{:016x}", *data),
                DerefStack => format!("{:016x}", *data),
                DerefHeapReg => format!("{:016x}", *data),
                TypeOp => "".to_string(),
                NoType => "".to_string()
            } + " "))
        }
        string
    }
}

#[derive(Debug, Clone)]
pub struct Byte {
    pub data: Box<u64>,
    pub pos: usize,
    pub tp: Types
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
            tp: Types::$tp
        }
    };
}
#[macro_export]
macro_rules! op {
    ($op:ident) => {
        {
            use $crate::bytecode::ops::Operations::*;
            Byte {
                data: Box::new($op as u64),
                pos: 0,
                tp: Types::TypeOp
            }
        }
    };
}
//macro to take a constant (u64) and return a Byte
#[macro_export]
macro_rules! constant {
    ($val:expr) => {
        Byte {
            data: Box::new($val),
            pos: 0,
            tp: Types::TypeU64
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
            tp: Types::TypeFunc
        }
    };
}
pub fn stringtohex(string: String) -> u64 {
    let mut hex = String::new();
    for c in string.chars() {
        hex.push_str(&(format!("{:02x}", c as u8)).trim());
    }
    u64::from_str_radix(&hex, 16).unwrap()
}

impl Byte {
    fn stringify(&self) -> String {
        format!("{:02x}", *(self.data))
    }
}