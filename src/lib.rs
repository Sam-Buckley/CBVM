#![allow(
    dead_code,
    unused_imports,
    unused_macros,
    unused_variables,
    unused_mut,
    unused_parens,
    unused_assignments,
    unused_braces,
    unused_import_braces
)]
pub mod builder;
pub mod bytecode;
pub mod reader;
use bytecode::{data::ByteData, ops::ArgType::*, ops::Operations::*, types::Types};
pub mod engine;
use builder::bytes::*;
use engine::memory::{Heap, Stack};
use std::time::Instant;
use std::{env, string};

fn main() {
    let mut engine = engine::Engine::new_with_size(8192);
    let mut bytestream = ByteStream::new();
    bytestream = bytestream
        .emit(op!(FUNC))
        .emitstream(stream!((TypeU64, 0x0)))
        .emit(op!(ALLOC))
        .emitstream(stream!((TypeReg, 0x1), (TypeU8, 5)))
        .emit(op!(STORE))
        .emitstream(stream!(
            (TypeU8, 0x1),
            (TypeU64, stringtohex("main\n".to_string()))
        ))
        .emit(op!(WRITE))
        .emitstream(stream!((TypeU8, 0x1), (TypeU8, 5)))
        .emit(op!(FLUSH))
        .emit(op!(FREE))
        .emitstream(stream!((TypeU8, 0x0)));
    std::fs::write("bytecode.cbvm", bytestream.stringify()).unwrap();
    let start = Instant::now();
    engine.run(bytestream.clone());
    println!("Time elapsed: {:?}", start.elapsed());
}

//create a macro to take a string and turn it into a ByteStream
//macro to take a string and return a ByteStream
