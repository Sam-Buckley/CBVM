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




//main function to take a filename and run the vm
