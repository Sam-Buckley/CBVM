mod builder;
mod bytecode;
use bytecode::{
    ops::ArgType::*,
    ops::Operations::*,
    types::Types,
    data::ByteData
};
mod engine;
use builder::{
    bytes::*,
};

fn main() {
    //use the builder macro to create a ByteStream
    println!("{:#?}", stream!{
        (TypeU64, 1),
        (TypeU64, 2),
        (TypeReg, 12)
    })
}