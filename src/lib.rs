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
use reader::Reader;
use std::str::from_utf8_unchecked;
use std::time::Instant;
use std::{env, string};



fn main() {
    let mut engine = engine::Engine::new_with_size(8192);
    // let mut bytestream = ByteStream::new();
    // bytestream = bytestream
    //     .emit(op!(FUNC))
    //     .emitstream(stream!((TypeU64, 0x0)))
    //     .emit(op!(ALLOC))
    //     .emitstream(stream!((TypeReg, 0x1), (TypeU8, 11)))
    //     .emit(op!(STORE))
    //     .emitstream(stream!(
    //         (TypeReg, 0x1),
    //         (TypeU8, 11)
    //     ))
    //     .emitstream(string!("hello world"))
    //     .emit(op!(WRITE))
    //     .emitstream(stream!((TypeU8, 0x1), (TypeU8, 11)))
    //     .emit(op!(FLUSH))
    //     .emit(op!(FREE))
    //     .emitstream(stream!((TypeU8, 0x1)));
    // // .emit(op!(JMP))
    // // .emitstream(stream!((TypeU8, 0x0)));;
    // std::fs::write("bytecode.cbvm", bytestream.stringify()).unwrap();
    // println!(
    //     "{:?}",
    //     bytestream
    //         .stringify()
    //         .chars()
    //         .map(|x| x as u8)
    //         .collect::<Vec<u8>>()
    // );
    // let start = Instant::now();
    let mut reader = Reader::new("bytecode.cbvm");
    reader.read();
    println!("{}", reader.stream.len());
    reader.group();
    println!("{}", reader.bytes);
    engine.run(reader.bytes);
}
