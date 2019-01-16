#[derive(Clone,Debug)]
pub struct Assembler {
    data: Vec<u8>,
}

use crate::native_x64::*;

use std::mem;

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            data: vec![],
        }
    }
    
    pub fn data<'r>(&'r self) -> &'r Vec<u8> {
        &self.data
    }

    

    pub fn emit(&mut self,op: u64,oplen: u64) {
        let mut slice: [u8;8] = unsafe {mem::transmute(op)};
        slice.reverse();
        let op = &slice[0..oplen as usize];
        for byte in op.iter() {
            self.data.push(*byte);
        }
    }

    pub fn emit32(&mut self,v: u32) {
        let slice: [u8;4] = unsafe {mem::transmute(v)};
        self.data.extend(&slice);
    }

    pub fn emit8(&mut self,op: u64,v: i64,oplen: u64) {
        self.emit(op | (v as u64) << 56,oplen);
    }

    pub fn emitrxb(&mut self,op: u64,r: u64,x: u64,b: u64) {
        self.emit(rexrxb(mod_rxb(op, r, x, b), r, x, b),oplen(op));
    }
    pub fn emitrr(&mut self,op: u64,r: u64,b: u64) {
        self.emit(rexrb(mod_rr(op, r, b), r, b),oplen(op));
    }
    pub fn emitrr8(&mut self,op: u64,r: u64,b: u64) {
        self.emit(rexrb8(mod_rr(op, r, b), r, b),oplen(op));
    }

    pub fn emitprr(&mut self,op: u64,r: u64,b: u64) {
        self.emit(rexprb(mod_rr(op, r, b), r, b),oplen(op));
    }
    
    pub fn emit_disp32_sib(o: u64,d: i32) -> u64 {
        (Assembler::emit_disp32( ((o)&0x00FFFFFFFFFFFFFFu64)<<8, d) >>8) | ((o)&0xFF00000000000000u64)
    }

    pub fn emit_disp32(mut op: u64,d: i32) -> u64 {
        if is_s8!(d) {
            assert!(((op>>56)&0xC0) == 0x80); // make sure mod bits == 2 == disp32 mode
            op = op ^ 0xC000000000000000u64;
        }

        return op;
    }

    pub fn emitrm8(&mut self,op: u64,r: u64,d: i32,b: u64) {
        self.emit(rexrb8(mod_disp32(op, r, b, d), r, b),oplen(op));
    }

    pub fn emitrm(&mut self,op: u64,r: u64,d: i32,b: u64) {
        self.emit(rexrb(mod_disp32(op, r, b, d), r, b),oplen(op));
    }

    pub fn emitrm_wide(&mut self,mut op: u64,r: u64,d: i32,b: u64) {
        op = Assembler::emit_disp32(op, d);
        self.emitrr(op, r, b);
        if is_s8!(d) {
            self.data.push(d as i8 as u8);
        } else {
            self.emit32(d as u32);
        }
    }

    pub fn emitprm(&mut self,mut op: u64,r: u64,d: i32,b: u64) {
        op = Assembler::emit_disp32(op, d);
        self.emitprr(op,r, b);
        if is_s8!(d) {
            self.data.push(d as i8 as u8);
        } else {
            self.emit32(d as u32);
        }
    }

    pub fn emitrm_imm32(&mut self,op: u64,b: u64,d: i32,imm: i32) {
        assert!(is_gp_reg(b));
        assert!(b & 7 != 4);
        self.emitrm_wide(op, RZero, d, b);
        self.emit32(imm as u32);
    }
    pub fn emitprm_imm16(&mut self,op: u64,b: u64,d: i32,imm: i32) {
        self.emitprm(op, RZero, d, b);
        let slice: [u8;2] = unsafe {mem::transmute(imm as i16)};
        self.data.extend(&slice);
    }

    pub fn emitrm_imm8(&mut self,op: u64,b: u64,d: i32,imm: i32) {
        self.emitrm_wide(op, RZero, d, b);
        self.data.push(imm as u8);
    }

    pub fn emitrr_imm(&mut self,op: u64,r: u64,b: u64,imm: i32) {
        assert!(is_gp_reg(r) && is_gp_reg(b));
        self.emitrr(op, r, b);
        self.emit32(imm as u32);
    }

    pub fn emitrr_imm8(&mut self,op: u64,r: u64,b: u64,imm: u8) {
        self.emitrr(op, r, b);
        self.data.push(imm);
    }

    pub fn emitprr_imm8(&mut self,op: u64,r: u64,b: u64,imm: u8) {
        assert!(is_gp_reg(r) && is_gp_reg(b) || is_fp_reg(r) && is_fp_reg(b));

        self.emitprr(op, r, b);
        self.data.push(imm);
    }
    pub fn emitr(&mut self,op: u64,r: u64) {
        self.emitrr(op, RZero, r);
    }
    pub fn emitr8(&mut self,op: u64,r: u64) {
        self.emitrr8(op, RZero, r);
    }

    pub fn emitr_imm64(&mut self,op: u64,r: u64,imm: u64) {
        self.emitr(op,r);
        let slice: [u8;8] = unsafe {mem::transmute(imm)};
        self.data.extend(&slice); 
    }
    pub fn emitrxb_imm(&mut self,op: u64,r: u64,x: u64,b: u64,imm: i32) {
        assert!(is_gp_reg(r) && is_gp_reg(x) && is_gp_reg(b));
        self.emitrxb(op,r,x,b);
            let slice: [u8;4] = unsafe {mem::transmute(imm)};
            self.data.extend(&slice);
    }

    pub fn emitr_imm8(&mut self,mut op: u64,b: u64,imm8: i32) {
        let len = oplen(op);
        assert!(is_gp_reg(b) && is_s8!(imm8));
        op |= (imm8 as u64) << 56 | (b & 7) << 48;
        self.emit(rexrb(op, RZero, b),len);
    }

    pub fn emitxm_abs(&mut self,mut op: u64,r: u64,addr32: i32) {
        let len = oplen(op);
        op = op | ((r & 7) << 3) << 48;
        op = rexrb(op,r,RZero);
        self.emit(op,len);
        self.emit32(addr32 as u32);
    }

    pub fn emitr_imm(&mut self,op: u64,r: u64,imm: i32) {
        self.emitrr_imm(op, RZero, r, imm);
    }
}