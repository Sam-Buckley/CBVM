#![allow(non_camel_case_types)]
mod regs;
mod callstack;
mod stdio;
pub mod memory;
use crate::{
    bytecode::{
        ops::ArgType::*,
        ops::Operations::*,
        types::Types,
        data::ByteData
    },
    builder::{
        bytes::*,
    },
    engine::memory::Heap,
};

type Register = u64;
type Address = u64;


type size_t = usize;
type reg_t = usize;


pub struct Engine {
    regs: regs::Registers,
    callstack: callstack::CallStack,
    heap: memory::Heap,
    stack: memory::Stack,
    io: stdio::IO,
    debug: bool,
}

/*
====================
    VM Functions
====================
*/
impl Engine {
    pub fn new() -> Self {
        Self {
            regs: regs::Registers::default(),
            callstack: callstack::CallStack::default(),
            heap: memory::Heap::default(),
            stack: memory::Stack::default(),
            io: stdio::IO::default(),
            debug: false,
        }
    }
    pub fn new_with_size(heap_size: size_t) -> Self {
        Self {
            regs: regs::Registers::default(),
            callstack: callstack::CallStack::default(),
            heap: memory::Heap::new(heap_size),
            stack: memory::Stack::default(),
            io: stdio::IO::default(),
            debug: false,
        }
    }
    pub fn move_reg(&mut self, reg: reg_t, value: u64) {
        self.regs[reg] = value;
    }
    fn alloc(&mut self, size: size_t, reg: reg_t) -> Address {
        let addr = self.heap.allocate(size).unwrap() as u64;
        self.move_reg(reg, addr);
        addr
    }
    fn free(&mut self, addr: Address) {
        self.heap.free(addr as usize).unwrap();
    }
    fn realloc(&mut self, addr: Address, size: size_t) -> Address {
        let addr = self.heap.realloc(addr as usize, size).unwrap() as u64;
        addr
    }
}