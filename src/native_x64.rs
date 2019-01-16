fast_const! {
    u64:
        RAX = 0,
        RCX = 1,
        RDX = 2,
        RBX = 3,
        RSP = 4,
        RBP = 5,
        RSI = 6,
        RDI = 7,
        R8  = 8,
        R9  = 9,
        R10 = 10,
        R11 = 11,
        R12 = 12,
        R13 = 13,
        R14 = 14,
        R15 = 15,

        XMM0 = 16,
        XMM1 = 17,
        XMM2 = 18,
        XMM3 = 19,
        XMM4 = 20,
        XMM5 = 21,
        XMM6 = 22,
        XMM7 = 23,
        XMM8 = 24,
        XMM9 = 25,
        XMM10 = 26,
        XMM11 = 27,
        XMM12 = 28,
        XMM13 = 29,
        XMM14 = 30,
        XMM15 = 31,

        FP = RBP,
        SP = RSP,
        RZero = 0,
        FirstRegNum = 0,
        LastRegNum = 31,
        UnknownRef = 32,
        UnspecifiedRef = 32,
        GpRegs = 0xffff,
        FpRegs = 0xffff0000
}

#[allow(non_snake_case)]
#[inline(always)]
pub const fn REGNUM(r: u64) -> u64 {
    r
}


pub const fn is_gp_reg(r: u64) -> bool {
    return (1 << r ) & GpRegs != 0;
}

pub const fn is_fp_reg(r: u64) -> bool {
    return (1<<r) & FpRegs != 0;
}


#[cfg(target_family="windows")]
pub const SavedRegs: u64 = 1<<REGNUM(RBX) | 1<<REGNUM(RSI) | 1<<REGNUM(RDI) |
                                          1<<REGNUM(R12) | 1<<REGNUM(R13) | 1<<REGNUM(R14) |
                                          1<<REGNUM(R15);

#[cfg(target_family="windows")]
pub const NumSavedRegs: u64 = 7;
#[cfg(target_family="windows")]
pub const NumArgRegs: u64 = 4;
#[cfg(target_family="unix")]
pub const SavedRegs: u64 = 1<<REGNUM(RBX) | 1<<REGNUM(R12) | 1<<REGNUM(R13) |
                                          1<<REGNUM(R14) | 1<<REGNUM(R15);
#[cfg(target_family="unix")]
pub const NumArgRegs: u64 =6;
pub const NumSavedRegs: u64 = 5;

#[cfg(target_family="unix")]
pub const ArgRegs: [u64;6] = [RDI,RSI,RDX,RCX,R8,R9];
#[cfg(target_family="unix")]
pub const savedRegs: [u64;5] = [RBX,R12,R13,R14,R15];

use crate::rmask;

pub const BaseRegs: u64 = GpRegs & !rmask(R12);
/// encode 2-register rex prefix.  dropped if none of its bits are set.

pub const fn oplen(op: u64) -> u64 {
    return op & 255;
}

pub fn rexrb(op: u64,r: u64,b: u64) -> u64 {
    let shift = 64 - 8 * oplen(op);
    assert!((((op>>shift)&255) == 0x40) || 
                    (((op>>shift)&255) == 0x48) );
    let rex = ((op >> shift) & 255) | ((r & 8) >> 1) | ((b & 8) >> 3);
    return if rex != 0x40 {
        op | rex << shift
    } else {
        op - 1
    }
}
/// encode 3-register rex prefix.  dropped if none of its bits are set.

pub fn rexrxb(op: u64,r: u64,x: u64,b: u64) -> u64 {
    let shift = 64 - 8 * oplen(op);
    let rex = ((op >> shift) & 255) | ((REGNUM(r)&8)>>1) | ((REGNUM(x)&8)>>2) | ((REGNUM(b)&8)>>3);
    return if rex != 0x40 {
        op | rex << shift
    } else {
        op - 1
    }
}

/// encode 2-register rex prefix.  dropped if none of its bits are set, but
/// keep REX if b >= rsp, to allow uniform use of all 16 8bit registers

pub fn rexrb8(op: u64,r: u64,b: u64) -> u64 {
    let shift = 64 - 8 * oplen(op);
    let rex = ((op >> shift) & 255) | ((REGNUM(r)&8)>>1) | ((REGNUM(b)&8)>>3);
    return if (rex | (REGNUM(b) & !3)) != 0x40 {
        op | rex << shift
    } else {
        op - 1
    }
}
/// encode 2-register rex prefix that follows a manditory prefix (66,F2,F3)
/// [prefix][rex][opcode]

pub fn rexprb(op: u64,r: u64,b: u64) -> u64 {
    let shift = 64 - 8 * oplen(op) * 8;
    if cfg!(debug_assertions) {
        let prefix = (op >> (shift - 8)) & 255;
        assert!(prefix != 0);
        assert!(prefix == 0x66 || prefix == 0xf2 || prefix == 0xf3);
    }

    let rex = ((op >> shift) & 255) | ((REGNUM(r)&8)>>1) | ((REGNUM(b)&8)>>3);
    // to drop rex, we replace rex with manditory prefix, and decrement length
    return if rex != 0x40 {
        op | rex << shift
    } else {
        ((op & !(255u64<<shift)) | (op>>(shift-8)&255) << shift) - 1
    }
}

/// [rex][opcode][mod-rr]

pub const fn mod_rr(op: u64,r: u64,b: u64) -> u64 {
    return op | ((REGNUM(r)&7)<<3 | (REGNUM(b)&7)) << 56;
}

/// [rex][opcode][modrm=r][sib=xb]

pub const fn mod_rxb(op: u64,r: u64,x: u64,b: u64) -> u64 {
    return op | /* modrm */((REGNUM(r)&7)<<3)<<48 | /*sib*/((REGNUM(x)&7)<<3|(REGNUM(b)&7))<<56
}


pub fn mod_disp32(mut op: u64,r: u64,b: u64,d: i32) -> u64 {
    assert!(is_gp_reg(r) && is_gp_reg(b));
    assert!(b & 7 != 4); // using RSP or R12 as base requires SIB
    let mod_ = ((op>>24)&255)>>6;
    if mod_ == 2 && is_s8!(d) {
        // op is:  0x[disp32=0][mod=2:r:b][op][rex][len]
        let len = oplen(op);
        op = (op & !0xff000000u64) | (0x40 | (REGNUM(r)&7)<<3 | (REGNUM(b)&7))<<24; // replace mod
        return op << 24 | ((d as i64) << 56) as u64 | (len - 3);
    } else {
        return op | ((d as i64)<<32) as u64 | ((REGNUM(r)&7)<<3 | (REGNUM(b)&7))<<24;
    }
}

use crate::assembler::Assembler;
use crate::opcodes::*;

macro_rules! one_reg {
    ($($op_name:ident : $op:ident)+) => {
        $(pub fn $op_name(&mut self,r: u64) {
            self.emitr($op,r);
        })+
    };
}

macro_rules! two_reg {
    ($($op_name:ident : $op:ident)+) => {
        $(pub fn $op_name(&mut self,r: u64,b: u64) {
            self.emitrr($op,r,b);
        })+
    };
}

impl Assembler {
    pub fn pushr(&mut self,r: u64) {
        self.emitr(X64_pushr,r);
    }
    pub fn popr(&mut self,r: u64) {
        self.emitr(X64_popr,r);
    }

    pub fn not(&mut self,r: u64) {
        self.emitr(X64_not,r);
    }
    pub fn notq(&mut self,r: u64) {
        self.emitr(X64_notq,r);
    }

    pub fn neg(&mut self,r: u64) {
        self.emitr(X64_neg,r);
    }
    pub fn negq(&mut self,r: u64) {
        self.emitr(X64_negq,r);
    }

    pub fn idiv(&mut self,r: u64) {
        self.emitr(X64_idiv,r);
    }
    pub fn idivq(&mut self,r: u64) {
        self.emitr(X64_idivq,r);
    }

    pub fn sete(&mut self,r: u64) {
        self.emitr8(X64_sete, r);
    }
    pub fn setl(&mut self,r: u64) {
        self.emitr8(X64_setl, r);
    }
    pub fn setle(&mut self,r: u64) {
        self.emitr8(X64_setle,r);
    }

    one_reg! {
        setg: X64_setg
        setge: X64_setge
        setb: X64_setb
        setbe: X64_setbe
        seta: X64_seta
        setae: X64_setae
        seto: X64_seto
    }

    two_reg! {
        addrr: X64_addrr
        subrr: X64_subrr
        andrr: X64_andrr
        orlrr: X64_orlrr
        xorrr: X64_xorrr
        imul : X64_imul
        imulq: X64_imulq
        cmplr: X64_cmplr
        movlr: X64_movlr
        addqrr: X64_addqrr
        subqrr: X64_subqrr
        andqrr: X64_andqrr
        orqrr: X64_orqrr
        xorqrr: X64_xorqrr
        cmpqr: X64_cmpqr
        movqr: X64_movqr
        movapsr: X64_movapsr
        unpcklps: X64_unpcklps
        cmovno: X64_cmovno
        cmovne: X64_cmovne
        cmovnl: X64_cmovnl
        cmovnle: X64_cmovnle
        cmovng: X64_cmovng
        cmovnge: X64_cmovnge
        cmovnb: X64_cmovnb
        cmovnbe: X64_cmovnbe
        cmovna: X64_cmovna
        cmovnae: X64_cmovnae
        movsxdr: X64_movsxdr
    }

    pub fn movi(&mut self,r: u64,i: i32) {
        self.emitr_imm(X64_movi,r,i);
    }

    pub fn movqi(&mut self,r: u64,i: u64) {
        self.emitr_imm64(X64_movqi, r, i);
    }
}