extern crate peak_jit;

use capstone::prelude::*;
use peak_jit::assembler::Assembler;
use peak_jit::native_x64::*;

fn main() {
    let mut cs: Capstone = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Intel)
        .detail(true)
        .build().unwrap();


    let mut asm = Assembler::new();
    
    asm.movi(RAX,0);
    let code = asm.data();
    println!("{:?}",code);
    let i = cs.disasm_all(code, 0x0).unwrap();
    for i in i.iter() 
    {
        println!("{}",i);
    }

}
