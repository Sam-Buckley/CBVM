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
    let args: Vec<String> = env::args().collect();
    let mut reader = Reader::new(&args[1]);
    reader.read();
    reader.group();
    let mut engine = engine::Engine::new();
    let start = Instant::now();
    engine.run(reader.bytes);
    let duration = start.elapsed();
    println!("\nTime elapsed in running VM is: {:?}", duration);
}