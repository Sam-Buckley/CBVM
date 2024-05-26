#![allow(non_camel_case_types)]
mod callstack;
pub mod memory;
mod regs;
mod stdio;

use crate::{
    builder::bytes::*,
    bytecode::{
        data::ByteData,
        ops::ArgType::*,
        ops::Operations::{self, *},
        ops::*,
        types::Types,
    },
    engine::memory::Heap,
};
use callstack::FnCall;
use stdio::IO;

type Register = u64;
type Address = u64;

type type_t = u8;
type size_t = usize;
type reg_t = usize;

pub struct Engine {
    accumulator: u64,
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
    pub fn alloc(&mut self, size: size_t, reg: reg_t) -> Address {
        let addr = self.heap.allocate(size).unwrap() as u64;
        self.move_reg(reg, addr);
        addr
    }
    fn free(&mut self, addr: Address) {
        self.heap.free(addr as usize).unwrap();
    }
    pub fn move_reg(&mut self, reg: reg_t, value: u64) {
        self.regs[reg] = value; // optimized
    }
    pub fn new() -> Self {
        Self {
            accumulator: 0,
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
            accumulator: 0,
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
    fn realloc(&mut self, addr: Address, size: size_t) -> Address {
        self.heap.realloc(addr as usize, size).unwrap() as u64
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

/*
====================
    VM Execution
====================
*/
impl Engine {
    pub fn run(&mut self, bytes: ByteStream) {
        self.data = bytes.clone();
        //find all TypeFuncs and store them in a jumptable
        for i in 0..bytes.bytes.len() {
            let byte = bytes.bytes[i].clone();
            match byte.tp {
                Types::TypeFunc => {
                    self.jumptable.push(i)
                },
                _ => (),
            }
        }
        println!("Jumptable: {:?}", self.jumptable);
        //iterate through the bytes and pass them to a handler
        while self.ip < bytes.bytes.len() {
            let byte = bytes.bytes[self.ip].clone();
            self.handle(byte);
        }
    }
    pub fn debug(&mut self, bytes: ByteStream) {
        self.debug = true;
        self.run(bytes);
    }
    fn handle(&mut self, byte: Byte) {
        let op: Operations = Operations::from(byte);
        self.ip += 1;
        match op {
            NOP => {
                ()
            }
            ADD => {
                let mut args = self.get_args(&MATH_OP_ARGS);
                let left = args[0] as u64;
                let right = args[1] as u64;
                self.accumulator =  left + right
            }
            SUB => {
                let mut args = self.get_args(&MATH_OP_ARGS);
                let left = args[0];
                let right = args[1];
                self.accumulator = (left - right) as u64;
            }
            WRITE => {
                let args = self.get_args(&IO_OUT_OP_ARGS);
                let reg = args[0];
                let size = args[1];
                let to_write = self.heap.read(reg as usize, size).unwrap();
                self.io.write(&to_write);
            }
            FLUSH => {
                self.io.flush();
            }
            STORE => {
                let args = self.get_args(&STORE_OP_ARGS);
                let addr = args[0];
                let len = args[1];
                //read as many args as the length
                for i in 0..len {
                    let byte = self.read_byte().unwrap();
                    let res = self.heap.write(addr as usize + i, byte as u8);
                }
            }
            LOAD => {
                let args = self.get_args(&LOAD_OP_ARGS);
                let addr = args[0];
                let location = args[1];
                let data = self.heap.read(location as usize, 1).unwrap();
                self.move_reg(addr, data[0] as u64);
            }
            FUNC => {
                let jp = self.read_byte().unwrap();
                self.jumptable.push(jp as usize);
            }
            ALLOC => {
                let mut args = self.get_args(&ALLOC_ARGS);
                let reg = args[0];
                let size = args[1];
                let ptr = self.alloc(size, reg);
                self.move_reg(reg, ptr);
            }
            FREE => {
                let args = self.get_args(&FREE_ARGS);
                let addr = args[0];
                self.free(addr as u64);
            }
            JMP => {
                let args = self.get_args(&JMP_ARGS);
                let addr = args[0];
                self.ip = addr;
            }
            CALL => {
                let args = self.get_args(&CALL_OP_ARGS);
                let addr = args[0];
                self.callstack.push(FnCall { ret: self.ip as u8 });
                self.ip = addr;
            }
            RET => {
                let ret = self.callstack.pop().unwrap();
                self.ip = ret.ret as usize;
            }
            MOV => {
                let args = self.get_args(&REG_OP_ARGS);
                let addr = args[0];
                let value = args[1];
                self.move_reg(addr, value as u64);
            }
            WRACC => {
                let args = self.get_args(&WRACC_ARGS);
                let addr = args[0];
                let value = self.accumulator;
                self.move_reg(addr, value);
            }
            REACC => {
                let args = self.get_args(&RDACC_ARGS);
                let addr = args[0];
                let value = self.regs[addr];
                self.accumulator = value;
            }
            PUSH => {
                let args = self.get_args(&PUSH_OP_ARGS);
                let reg = args[0];
                let value = self.regs[reg];
                self.stack.push(value as u8);
            }
            POP => {
                let args = self.get_args(&REG_OP_ARGS);
                let reg = args[0];
                let value = self.stack.pop() as u64;
                self.move_reg(reg, value);
            }
            _ => println!("Invalid opcode: {:?}", op),
        };
    }
    fn get_args(&mut self, args: &[ArgType]) -> Vec<usize> {
        let mut regs = Vec::new();
        for arg in args {
            match *arg {
                //remove reference, reduce indirection
                Typed => {
                    let byte = self.read_byte();
                    let tpr = self.handle_typed(byte);
                    regs.push(tpr);
                }
                Untyped => {
                    let value = self.read_byte().unwrap();
                    regs.push(value as usize);
                }
                Dest => {
                    let reg = self.read_byte().unwrap();
                    regs.push(reg as usize);
                }
                Func => {
                    let func = self.read_byte().unwrap();
                    regs.push(func as usize);
                }
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
    fn handle_typed(&mut self, byte: Byte) -> usize {
        let tp = byte.tp;
        let byte = byte.unwrap() as usize;
        use Types::*;
        match tp {
            TypeU8 => byte,
            TypeU64 => byte,
            TypeI128 => byte,
            TypeU128 => byte,
            TypeF32 => byte,
            TypeF64 => byte,
            DerefStack => self.stack.get(byte) as usize,
            DerefHeapReg => {
                let rg = self.regs[byte];
                self.heap.read(rg as usize, 1).unwrap()[0] as usize
            }
            TypeI8 => byte,
            TypeAddr => byte,
            TypeReg => self.regs.data[byte] as usize,
            TypeI64 => byte,
            NoType => byte,
            TypeFunc => self.jumptable[byte],
            _ => panic!("Invalid type: {:?}", tp),
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
