#![allow(non_camel_case_types)]
mod regs;
mod callstack;
mod stdio;
pub mod memory;

use crate::{
    bytecode::{
        ops::*,
        ops::ArgType::*,
        ops::Operations::{self, *},
        types::Types,
        data::ByteData
    },
    builder::{
        bytes::*,
    },
    engine::memory::Heap,
};
use stdio::IO;

type Register = u64;
type Address = u64;

type type_t = u8;
type size_t = usize;
type reg_t = usize;


pub struct Engine {
    pub regs: regs::Registers,
    callstack: callstack::CallStack,
    pub heap: memory::Heap,
    stack: memory::Stack,
    io: stdio::IO,
    debug: bool,
    ip: usize,
    data: ByteStream,
    jumptable: Vec<usize>,
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
            ip: 0,
            data: ByteStream::new(),
            jumptable: Vec::new(),
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
            ip: 0,
            data: ByteStream::new(),
            jumptable: Vec::new(),
        }
    }
    pub fn move_reg(&mut self, reg: reg_t, value: u64) {
        self.regs[reg] = value;
    }
    pub fn alloc(&mut self, size: size_t, reg: reg_t) -> Address {
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

/*
====================
    VM Execution
====================
*/
impl Engine {
    pub fn run (&mut self, bytes: ByteStream) {
        self.data = bytes.clone();
        //iterate through the bytes and pass them to a handler
       while self.ip < bytes.bytes.len() {
            let byte = bytes.bytes[self.ip].clone();
            self.handle(byte);
        }
    }
    fn handle(&mut self, byte: Byte) {
        let op: Operations = Operations::from(byte);
        match op {
            ADD => {
                self.ip += 1;
                //let mut args = self.get_args(&MATH_OP_ARGS);
                let dest = self.read_byte().unwrap();
                let left = self.read_byte().unwrap();
                let right = self.read_byte().unwrap();
                self.move_reg(dest as usize, left+right)
            },
            SUB => {
                let mut args = self.get_args(&MATH_OP_ARGS);
                let dest = args[0];
                let left = args[1];
                let right = args[2];
                self.regs[dest] = (left - right) as u64;
            },
            WRITE => {
                self.ip += 1;
                let data = self.read_byte().unwrap();
                let size = self.read_byte().unwrap();
                let buffer = self.read_byte().unwrap();
                let to_write = self.heap.read(data as usize, size as usize).unwrap();
                self.io.write(&to_write);
                self.ip -= 1;
            },
            FLUSH => {
                self.ip += 1;
                self.io.flush();
            },
            STORE => {
                self.ip += 1;
                //let args = self.get_args(&[Typed]);
                let ptr = self.read_byte().unwrap();
                let data = self.read_byte().unwrap();
                let bytes = hextostring(data).iter().filter(|&&x| x != 0).cloned().collect::<Vec<u8>>();
                for (i, byte) in bytes.iter().enumerate() {
                    self.heap.write(ptr as usize+i, *byte).unwrap();
                }
            },
            FUNC => {
                self.ip += 1;
                let mut args = self.get_args(&[Untyped]);
                let name = args[0];
                self.jumptable.push(name);
                self.ip = name;
            },
            ALLOC => {
                self.ip += 1;
                let mut args = self.get_args(&ALLOC_ARGS);
                let reg = args[0];
                let size = args[1];
                let ptr = self.alloc(size, reg);
                self.move_reg(reg, ptr);
                self.ip -= 1;
            },
            _ => todo!()
        }
    }
    fn get_args(&mut self, args: &[ArgType]) -> Vec<usize> {
        let mut regs = Vec::new();
        for arg in args {
            //match the arg type, typed = read 2 bytes (type, value)
            //untyped = read 1 byte (value)
            //dest = read 1 byte (reg)
            //func = read next byte (func)
            //then push the value to the regs vec
            match arg {
                Typed => {
                    let type_byte = self.read_byte();
                    let value = self.read_byte();
                    let tpr = self.handle_typed(type_byte, value);
                    regs.push(tpr);
                },
                Untyped => {
                    let value = self.read_byte().unwrap();
                    regs.push(value as usize);
                },
                Dest => {
                    let reg = self.read_byte().unwrap();
                    regs.push(reg as usize);
                },
                Func => {
                    let func = self.read_byte().unwrap();
                    regs.push(func as usize);
                },
            }
        }
        regs
    }
    fn read_byte(&mut self) -> Byte {
        if self.ip >= self.data.bytes.len() {
            self.ip = self.data.bytes.len() - 1;
        }
        let byte = self.data.bytes[self.ip].clone();
        self.ip += 1;
        byte
    }
    fn current_byte(&mut self) -> Byte {
        self.data.bytes[self.ip].clone()
    }
    fn previous_byte(&mut self) -> Byte {
        self.data.bytes[self.ip - 1].clone()
    }
    fn handle_typed(&mut self, tp: Byte, byte: Byte) -> usize {
        let tp = tp.tp;
        let byte = *(byte.data).clone() as usize;
        use Types::*;
        return match tp{
            TypeU8 => byte,
            TypeU64 => byte,
            TypeI128 => byte,
            TypeU128 => byte,
            TypeF32  => byte,
            TypeF64  => byte,
            DerefStack => {
                self.stack.get(byte) as usize
            },
            DerefHeapReg => {
                let rg = self.regs[byte];
                self.heap.read(rg as usize, 1).unwrap()[0] as usize
            },
            TypeI8 => byte,
            TypeAddr => byte,
            TypeReg => self.regs.data[byte] as usize,
            TypeI64 => byte,
            NoType => byte,
            TypeFunc => self.jumptable[byte] as usize,
            _ => panic!("Invalid type: {:?}", tp)
        }
    }
}

pub fn hextostring(hex: u64) -> Vec<u8> {
    let hex_string = format!("{:016x}", hex);
    let bytes = hex_string.as_bytes();
    let mut result = String::new();

    for i in 0..bytes.len() / 2 {
        let byte = ((hex >> ((bytes.len() / 2 - i - 1) * 8)) & 0xFF) as u8;
        result.push(byte as char);
    }
    result.bytes().collect::<Vec<u8>>()
}