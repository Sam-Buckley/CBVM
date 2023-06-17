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
use std::{env, string};
use engine::memory::{Heap, Stack};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut engine = engine::Engine::new_with_size(16384);
    let mut bytestream = ByteStream::new();
    bytestream = bytestream.emit(op!(FUNC)).emitstream(stream!((TypeU64, stringtohex("start".to_string()))))
        .emit(op!(ALLOC))
        .emitstream(stream!((TypeReg, 0x1),(TypeU8, 5)))
        .emit(op!(STORE))
        .emitstream(stream!((TypeReg, 0x1),(TypeU64, stringtohex("main\n".to_string()))))
        .emit(op!(WRITE))
        .emitstream(stream!((TypeReg, 0x1), (TypeU8, 4)))
        .emit(op!(FLUSH));
    engine.alloc(12, 0x0);
    engine.run(bytestream.clone());
    std::fs::write("bytecode.cbvm", bytestream.stringify()).unwrap();
}

//create a macro to take a string and turn it into a ByteStream
//macro to take a string and return a ByteStream
