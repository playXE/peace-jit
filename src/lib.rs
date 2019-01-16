#![allow(non_upper_case_globals)]

#[macro_use]
pub mod macros;
pub mod opcodes;
pub mod native_x64;
pub mod assembler;

pub const fn rmask(reg: u64) -> u64 {
    1 << reg
}
