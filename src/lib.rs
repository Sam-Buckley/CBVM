#![allow(dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_parens, unused_assignments, unused_braces, unused_import_braces)]
mod builder;
mod bytecode;
use bytecode::{
    ops::ArgType::*,
    ops::Operations::*,
    types::Types,
    data::ByteData
};
mod engine;
use builder::{
    bytes::*,
};
use engine::memory::Heap;

fn main() {
    let mut bstream: ByteStream = ByteStream::new()
        .emit(func!("dave"))
        .emit(byte!((TypeOp, 0x19)))
        .emitstream(stream!{
            (TypeU8, 0x01),
            (TypeU64, 0xabc),
            (TypeAddr, 0x13)
        });
    std::fs::write("bytecode.txt", bstream.stringify()).expect("Unable to write file");
}

//create a macro to take a string and turn it into a ByteStream
//macro to take a string and return a ByteStream
