
#[repr(u8)]
#[derive(Debug)]
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
    //IO
    WRITE = 0x19,
    READ = 0x1A,

    //registers
    MOV = 0x1B,

    //functions
    FUNC = 0x64,
    RET = 0x65,
    CALL = 0x66,
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
pub enum ArgType {
    Typed, Untyped, Dest, Func
}
use ArgType::*;

pub const MATH_OP_ARGS: [ArgType; 2] = [
    Dest, Typed
];

pub const BITWISE_OP_ARGS: [ArgType; 2] = [
    Dest, Typed
];
pub const COMPARISON_OP_ARGS: [ArgType; 2] = [
    Dest, Typed
];

pub const CONTROL_FLOW_OP_ARGS: [ArgType; 2] = [
    Func, Typed //Func, condition
];

pub const LOAD_OP_ARGS: [ArgType; 2] = [
    Dest, Typed //Reg, size
];
pub const STORE_OP_ARGS: [ArgType; 2] = [
    Typed, Typed //Data, size
];

pub const IO_OP_ARGS: [ArgType; 3] = [
    Typed, Typed, Typed //Data, Size, Buffer
];

pub const REG_OP_ARGS: [ArgType; 2] = [
    Dest, Typed //Reg, Value
];

pub const CALL_OP_ARGS: [ArgType; 1] = [
    Func //Func
];