#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Types {
    TypeU8 = 0x00,
    TypeU64 = 0x01,
    TypeI8 = 0x02,
    TypeI64 = 0x03,
    TypeF32 = 0x04,
    TypeF64 = 0x05,
    TypeU128 = 0x06,
    TypeI128 = 0x07,
    //Address and register
    TypeAddr = 0x08,
    TypeReg = 0x09,
    TypeFunc = 0x0A,
    TypeOp = 0x0B,
    DerefStackReg = 0x0C,
    DerefHeapReg = 0x0D,
    NoType
}