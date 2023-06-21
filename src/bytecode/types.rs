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
    DerefStack = 0x0C,
    DerefHeapReg = 0x0D,
    DerefStackReg = 0x0E,
    NoType
}
impl From<u8> for Types {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Types::TypeU8,
            0x01 => Types::TypeU64,
            0x02 => Types::TypeI8,
            0x03 => Types::TypeI64,
            0x04 => Types::TypeF32,
            0x05 => Types::TypeF64,
            0x06 => Types::TypeU128,
            0x07 => Types::TypeI128,
            0x08 => Types::TypeAddr,
            0x09 => Types::TypeReg,
            0x0A => Types::TypeFunc,
            0x0B => Types::TypeOp,
            0x0C => Types::DerefStack,
            0x0D => Types::DerefHeapReg,
            0x0E => Types::DerefStackReg,
            _ => Types::NoType
        }
    }
}