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
use std::io::Write;
pub mod bytecode;
pub mod reader;
use bytecode::{data::ByteData, ops::ArgType::*, ops::Operations::*, types::Types};
pub mod engine;
use builder::bytes::*;
use engine::memory::{Heap, Stack};
use reader::Reader;
use std::io::Read;
use std::str::from_utf8_unchecked;
use std::time::Instant;
use std::{env, string};
pub mod asm;
use cbasm;


fn main() {

    //first arg is what cmd to do, second is path
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }
    let cmds = vec![
        "run".to_string(),
        "debug".to_string(),
        "help".to_string(),
        "view".to_string(),
        "asm".to_string(),
        "compile".to_string()
    
    ];
    //check first arg to be in list of cmds
    if cmds.contains(&args[1]) {
        match args[1].as_str() {
            "run" => run(),
            "debug" => debug(),
            "help" => help(),
            "view" => view(),
            "asm" => asm(),
            "compile" => compile(),
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
    println!("asm <path> - view asm");
    println!("compile <path> - compile to bytecode");
}

fn compile() {
    //take file path from cli
    let args: Vec<String> = env::args().collect();
    //read file
    let mut buf = Vec::new();
    let mut file = std::fs::File::open(&args[2]).unwrap();
    file.read_to_end(&mut buf).unwrap();
    let code = cbasm::build(String::from_utf8(buf).unwrap());
    //write to out.cb
    let mut file = std::fs::File::create("out.cb").unwrap();
    file.write_all(code.stringify().as_bytes()).unwrap();
}

//view function, take path from cli, read file, group bytes, and print bytes
fn view() {
    let args: Vec<String> = env::args().collect();
    let mut reader = Reader::new(&args[2]);
    reader.read();
    reader.group();
    println!("{}", reader.bytes);
}