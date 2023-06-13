#![allow(dead_code)]
mod bytecode;
mod engine;
use bytecode::ops::Operations::*;
mod builder;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("bytecode.txt")?;
    let reader = BufReader::new(file);

    // Process the line containing bytecode sets
    if let Some(line) = reader.lines().next() {
        let line = line?;
        let bytecodes: Vec<&str> = line.split(' ').collect();

        // Process each bytecode set
        for bytecode in bytecodes {
            // Parse the hexadecimal bytecode
            if let Ok(value) = u64::from_str_radix(bytecode, 16) {
                // Process the parsed bytecode
                println!("Parsed bytecode: {}", value);
                // Add your custom logic here to handle the bytecode
            } else {
                println!("Invalid bytecode: {}", bytecode);
            }
        }
    }

    Ok(())
}

