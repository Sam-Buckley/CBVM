extern crate alloc;
use alloc::vec::Vec;

pub type CallStack = Vec<FnCall>;
pub struct FnCall {
    ret: u8
}