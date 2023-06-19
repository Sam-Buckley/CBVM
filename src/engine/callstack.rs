#![allow(dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_parens, unused_assignments, unused_braces, unused_import_braces)]
extern crate alloc;
use alloc::vec::Vec;

pub type CallStack = Vec<FnCall>;
pub struct FnCall {
    pub ret: u8
}