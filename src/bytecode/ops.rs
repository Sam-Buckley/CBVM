
use crate::bytecode::data::ByteData;
use crate::builder::bytes::{Byte, ByteStream};
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Operations {
    NOP = 0x00,
    //Arithmetic
    ADD = 0x01,
    SUB = 0x02,
    MUL = 0x03,
    DIV = 0x04,
    MOD = 0x05,
    //Bitwise
    AND = 0x06,
    OR = 0x07,
    XOR = 0x08,
    NOT = 0x09,
    //Comparison
    EQ = 0x0A,
    NEQ = 0x0B,
    LT = 0x0C,
    GT = 0x0D,

    //Stack
    PUSH = 0x0E,
    POP = 0x0F,
    DUP = 0x10,
    SWAP = 0x11,
    //Control Flow
    JMP = 0x12,
    JZ = 0x13,
    JNZ = 0x14,
    //Memory
    LOAD = 0x17,
    STORE = 0x18,
    ALLOC = 0x1C,
    FREE = 0x1D,
    REALLOC = 0x1E,
    //IO
    WRITE = 0x19,
    READ = 0x1A,
    FLUSH = 0x1F,

    //registers
    MOV = 0x1B,
    INC = 0x15,
    DEC = 0x16,
    
    //functions
    FUNC = 0x64,
    RET = 0x65,
    CALL = 0x66,
    
    //accumulator
    WRACC = 0x67,
    REACC = 0x68,
}

impl From<Byte> for Operations {
    fn from(byte: Byte) -> Self {
        match *(byte.data) as u8 {
            0x00 => Operations::NOP,
            0x01 => Operations::ADD,
            0x02 => Operations::SUB,
            0x03 => Operations::MUL,
            0x04 => Operations::DIV,
            0x05 => Operations::MOD,
            0x06 => Operations::AND,
            0x07 => Operations::OR,
            0x08 => Operations::XOR,
            0x09 => Operations::NOT,
            0x0A => Operations::EQ,
            0x0B => Operations::NEQ,
            0x0C => Operations::LT,
            0x0D => Operations::GT,
            0x0E => Operations::PUSH,
            0x0F => Operations::POP,
            0x10 => Operations::DUP,
            0x11 => Operations::SWAP,
            0x12 => Operations::JMP,
            0x13 => Operations::JZ,
            0x14 => Operations::JNZ,
            0x17 => Operations::LOAD,
            0x18 => Operations::STORE,
            0x19 => Operations::WRITE,
            0x1A => Operations::READ,
            0x1B => Operations::MOV,
            0x15 => Operations::INC,
            0x16 => Operations::DEC,
            0x1C => Operations::ALLOC,
            0x1D => Operations::FREE,
            0x1E => Operations::REALLOC,
            0x1F => Operations::FLUSH,
            0x64 => Operations::FUNC,
            0x65 => Operations::RET,
            0x66 => Operations::CALL,
            0x67 => Operations::WRACC,
            0x68 => Operations::REACC,
            _ => panic!("Invalid opcode: {}", *(byte.data) as u8)
        }
    }
}
impl From<u8> for Operations {
    fn from(code: u8) -> Operations {
        match code {
            0x00 => Operations::NOP,
            0x01 => Operations::ADD,
            0x02 => Operations::SUB,
            0x03 => Operations::MUL,
            0x04 => Operations::DIV,
            0x05 => Operations::MOD,
            0x06 => Operations::AND,
            0x07 => Operations::OR,
            0x08 => Operations::XOR,
            0x09 => Operations::NOT,
            0x0A => Operations::EQ,
            0x0B => Operations::NEQ,
            0x0C => Operations::LT,
            0x0D => Operations::GT,
            0x0E => Operations::PUSH,
            0x0F => Operations::POP,
            0x10 => Operations::DUP,
            0x11 => Operations::SWAP,
            0x12 => Operations::JMP,
            0x13 => Operations::JZ,
            0x14 => Operations::JNZ,
            0x17 => Operations::LOAD,
            0x18 => Operations::STORE,
            0x19 => Operations::WRITE,
            0x1A => Operations::READ,
            0x1B => Operations::MOV,
            0x15 => Operations::INC,
            0x16 => Operations::DEC,
            0x1C => Operations::ALLOC,
            0x1D => Operations::FREE,
            0x1E => Operations::REALLOC,
            0x1F => Operations::FLUSH,
            0x64 => Operations::FUNC,
            0x65 => Operations::RET,
            0x66 => Operations::CALL,
            0x67 => Operations::WRACC,
            0x68 => Operations::REACC,
            _ => panic!("Invalid opcode: {}", code)
        }
    }
}

impl ByteData for Operations {
    fn get(&self) -> u8 {
        self.clone() as u8
    }
    fn set(&mut self, data: u8) {
        *self = unsafe { core::mem::transmute(data) };
    }
}

pub enum MathOperationSides {
    ConstConst = 0,
    ConstReg = 1,
    RegConst = 2,
    RegReg = 3,
}
pub enum MathOpTypes {
    Signed = 0,
    Unsigned = 1,
    Float = 2,
}
#[derive(Debug, Clone, Copy)]
pub enum ArgType {
    Typed, Untyped, Dest, Func,
}
use ArgType::*;

pub const MATH_OP_ARGS: [ArgType; 2] = [
    Typed, Typed
];

pub const JMP_ARGS: [ArgType; 1] = [
    Typed
];
pub const REACC_ARGS: [ArgType; 1] = [
    Dest
];
pub const WRACC_ARGS: [ArgType; 1] = [
    Typed
];
pub const BITWISE_OP_ARGS: [ArgType; 2] = [
    Dest, Typed
];
pub const COMPARISON_OP_ARGS: [ArgType; 2] = [
    Typed, Typed
];

pub const ALLOC_ARGS : [ArgType; 2] = [
    Dest, Typed
];
pub const REALLOC_ARGS : [ArgType; 2] = [
    Dest, Typed
];
pub const FREE_ARGS : [ArgType; 1] = [
    Typed
];

pub const CONTROL_FLOW_OP_ARGS: [ArgType; 1] = [
    Func //Func, condition
];

pub const LOAD_OP_ARGS: [ArgType; 2] = [
    Dest, Typed //Reg, size
];
pub const STORE_OP_ARGS: [ArgType; 2] = [
    Typed, Typed //Address, Data
];

pub const IO_OUT_OP_ARGS: [ArgType; 2] = [
    Typed, Typed//Data, Size, Buffer
];

pub const IO_IN_OP_ARGS: [ArgType; 2] = [
    Typed, Typed //Address, Buffer, Size if not specified will run until oob
];

pub const REG_OP_ARGS: [ArgType; 2] = [
    Dest, Typed //Reg, Value
];

pub const CALL_OP_ARGS: [ArgType; 1] = [
    Func //Func
];

pub const PUSH_OP_ARGS: [ArgType; 1] = [
    Typed //Value
];


pub const INC_OP_ARGS: [ArgType; 1] = [
    Dest //Reg
];
pub const DEC_OP_ARGS: [ArgType; 1] = [
    Dest //Reg
];