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
pub mod asm;


fn main() {
    //first arg is what cmd to do, second is path
    let args: Vec<String> = env::args().collect();
    let cmds = vec![
        "run".to_string(),
        "debug".to_string(),
        "help".to_string(),
        "view".to_string(),
    ];
    //check first arg to be in list of cmds
    if cmds.contains(&args[1]) {
        match args[1].as_str() {
            "run" => run(),
            "debug" => debug(),
            "help" => help(),
            "view" => view(),
            _ => println!("Invalid command"),
        }
    } else {
        help()
    }
}

//run function, take path from cli, read file, group bytes, and run vm
fn run() {
    let args: Vec<String> = env::args().collect();
    let mut reader = Reader::new(&args[2]);
    reader.read();
    reader.group();
    let mut engine = engine::Engine::new();
    let start = Instant::now();
    engine.run(reader.bytes);
    let duration = start.elapsed();
    println!("\nTime elapsed in running VM is: {:?}", duration);
}

fn asm () {
    let args: Vec<String> = env::args().collect();
    let mut reader = Reader::new(&args[2]);
    reader.read();
    reader.group();
    println!("{}", asm::mkasm(reader.bytes));
}

//debug function, take path from cli, read file, group bytes, and run vm with debug
fn debug() {
    let args: Vec<String> = env::args().collect();
    let mut reader = Reader::new(&args[2]);
    reader.read();
    reader.group();
    let mut engine = engine::Engine::new();
    let start = Instant::now();
    engine.debug(reader.bytes);
    let duration = start.elapsed();
    println!("\nTime elapsed in running VM is: {:?}", duration);
}

//help function, print help
fn help() {
    println!("Commands:");
    println!("run <path> - run vm");
    println!("debug <path> - run vm with debug");
    println!("help - print help");
    println!("view <path> - view bytecode");
}

//view function, take path from cli, read file, group bytes, and print bytes
fn view() {
    let args: Vec<String> = env::args().collect();
    let mut reader = Reader::new(&args[2]);
    reader.read();
    reader.group();
    println!("{}", reader.bytes);
}