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
use engine::memory::{Heap, Stack};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut stack = Stack::new();
    let mut heap = Heap::new(8192);
    //push a dynamic string to the heap and make a pointer to it on the stack
    let string = "Hello, World!\0";
    let string_ptr = heap.allocate(string.len()).unwrap();
    for (i, byte) in string.bytes().enumerate() {
        heap.write(string_ptr + i, byte).unwrap();
    }
    stack.push(string_ptr as u8);
    println!("string: {}", heap.read(stack.peek() as usize, string.len()).unwrap().iter().map(|&i| i as char).collect::<String>());
    //print benchmark
    println!("{}ms", start.elapsed().as_millis());
}

//create a macro to take a string and turn it into a ByteStream
//macro to take a string and return a ByteStream
