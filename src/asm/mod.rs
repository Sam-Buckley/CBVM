//take a ByteStream and turn into asm, run through the stream and emit the asm, every FuncType is a new line, and thrn the ops and then args
use crate::bytecode::types::Types::{*};
use crate::builder::bytes::ByteStream;
use crate::bytecode::ops::Operations::{self, *};
//import stream macro
use crate::{stream, byte, typed, Byte, Types};

pub fn mkasm(stream: ByteStream) -> String {
    println!("{:?}", stream.bytes.len());
    let mut asm = String::new();
    for byte in stream.bytes {
        match byte.tp {
            TypeOp => {
                asm.push_str(&format!("\n{:?} ", Operations::from(*(byte.data) as u8)));
            },
            TypeReg => {
                asm.push_str(&format!("[{:x}] ", *(byte.data)));
            },
            TypeU64 => {
                asm.push_str(&format!("64u{:x} ", *(byte.data)));
            },
            TypeU8 => {
                asm.push_str(&format!("8u{:x} ", *(byte.data)));
            }
            TypeFunc => {
                asm.push_str(&format!(":{:?} ", byte.data));
            },
            TypeAddr => {
                asm.push_str(&format!("@{:x} ", *(byte.data)));
            },
            TypeI64 => {
                asm.push_str(&format!("64i{:x} ", *(byte.data)));
            },
            TypeI128 => {
                asm.push_str(&format!("128i{:x} ", *(byte.data)));
            },
            TypeU128 => {
                asm.push_str(&format!("128u{:x} ", *(byte.data)));
            },
            TypeF32 => {
                asm.push_str(&format!("32f{:x} ", *(byte.data)));
            },
            TypeF64 => {
                asm.push_str(&format!("64f{:x} ", *(byte.data)));
            },
            DerefStack => {
                asm.push_str(&format!("({:x}) ", *(byte.data)));
            },
            DerefHeapReg => {
                asm.push_str(&format!("h{:x}", *(byte.data)));
            },
            DerefStackReg => {
                asm.push_str(&format!("s{:x} ", *(byte.data)));
            },
            NoType => {
                asm.push_str(&format!("{:x} ", *(byte.data)));
            }
            TypeI8 => {
                asm.push_str(&format!("{:x} ", *(byte.data)));
            }
            TypeJmp => {
                asm.push_str(&format!("j{:x} ", *(byte.data)));
            }
        }
    }
    asm
}
//function to reverse mkasm
pub fn rvasm(asm: String) -> ByteStream {
    let mut stream = ByteStream::new();
    let mut data = Vec::new();
    for c in asm.chars() {
        data.push(c as u8);
    }
    let mut pos = 0;
    while pos < data.len() {
        let tp = Types::from(data[pos]);
        pos += 1;
        let arg = data[pos] as u64;
        pos += 1;
        let byte = Byte{
            data: Box::new(arg),
            pos: 0,
            tp: tp,
        };
        stream.emit(byte);
    }
    stream
}
