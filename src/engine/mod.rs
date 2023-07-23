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
        self.heap.realloc(addr as usize, size).unwrap() as u64
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
        //iterate through the bytes and pass them to a handler
        while self.ip < bytes.bytes.len() {
            let byte = bytes.bytes[self.ip].clone();
            self.handle(byte);
        }
    }
    fn handle(&mut self, byte: Byte) {
        let op: Operations = Operations::from(byte);
        self.ip += 1;
        match op {
            ADD => {
                let mut args = self.get_args(&MATH_OP_ARGS);
                let dest = args[0];
                let left = args[1] as u64;
                let right = args[2] as u64;
                self.move_reg(dest, left + right)
            }
            SUB => {
                let mut args = self.get_args(&MATH_OP_ARGS);
                let dest = args[0];
                let left = args[1];
                let right = args[2];
                self.regs[dest] = (left - right) as u64;
            }
            WRITE => {
                let args = self.get_args(&IO_OUT_OP_ARGS);
                let reg = args[0];
                let size = args[1];
                let data = self.regs[reg];
                let to_write = self.heap.read(data as usize, size).unwrap();
                self.io.write(&to_write);
            }
            FLUSH => {
                self.io.flush();
            }
            STORE => {
                let args = self.get_args(&STORE_OP_ARGS);
                let reg = args[0];
                let data = hextostring(args[1] as u64)
                    .into_iter()
                    .filter(|x| *x != 0)
                    .collect::<Vec<u8>>();
                let ptr = self.regs[reg] as usize;
                (0..data.len()).for_each(|i| {
                    self.heap.write(ptr + i, data[i]).unwrap();
                });
            }
            FUNC => {
                let name = self.read_byte().unwrap();
                self.jumptable.push(name as usize);
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
                let reg = args[0];
                let addr = self.regs[reg];
                self.free(addr);
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
            _ => todo!(),
        };
    }
    fn get_args(&mut self, args: &[ArgType]) -> Vec<usize> {
        let mut regs = Vec::new();
        for arg in args {
            match arg {
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
