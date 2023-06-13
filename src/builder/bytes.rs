extern crate alloc;
use alloc::vec::Vec;
use crate::bytecode::{
    ops::ArgType::*,
    ops::Operations::*,
    types::Types,
    data::ByteData
};

#[derive(Debug, Clone)]
pub struct ByteStream {
    pos: usize,
    pub bytes: Vec<Byte>
}

impl ByteStream {
    pub fn new() -> ByteStream {
        ByteStream {
            pos: 0,
            bytes: Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Byte {
    pub data: Box<u64>,
    pub pos: usize,
    pub tp: Types
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

//macro to take a tuple and direct to the correct macro
#[macro_export]
macro_rules! handle {
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
    //rule should be a list of tuples
    ($(($($tp:ident, $val:expr),+)),+) => {
        {
            let mut stream = ByteStream::new();
            $(
                $(
                    stream.bytes.push(handle!(($tp, $val)));
                )+
            )+
            stream
        }
    };
}