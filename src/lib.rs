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
pub mod asm;
use bytecode::{data::ByteData, ops::ArgType::*, ops::Operations::*, types::Types};
pub mod engine;
use builder::bytes::*;
use engine::memory::{Heap, Stack};
use reader::Reader;
use std::str::from_utf8_unchecked;
use std::time::Instant;
use std::{env, string};

pub fn read (path: &str) -> Vec<u8> {
    let mut reader = Reader::new(path);
    reader.read();
    reader.group();
    reader.bytes.into()
}

pub fn run(bytes: Vec<u8>) {
    let mut engine = engine::Engine::new();
    let start = Instant::now();
    engine.run(bytes.into());
    let duration = start.elapsed();
    println!("\nTime elapsed in running VM is: {:?}", duration);
}

fn main(){
    
}