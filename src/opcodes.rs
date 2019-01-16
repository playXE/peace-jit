fast_const! {
u64:
        X64_addqrr  = 0xC003480000000003u64, // 64bit add r += b
        X64_addqri  = 0xC081480000000003u64, // 64bit add r += int64(immI)
        X64_addqr8  = 0x00C0834800000004u64, // 64bit add r += int64(imm8)
        X64_andqri  = 0xE081480000000003u64, // 64bit and r &= int64(immI)
        X64_andqr8  = 0x00E0834800000004u64, // 64bit and r &= int64(imm8)
        X64_orqri   = 0xC881480000000003u64, // 64bit or  r |= int64(immI)
        X64_orqr8   = 0x00C8834800000004u64, // 64bit or  r |= int64(imm8)
        X64_xorqri  = 0xF081480000000003u64, // 64bit xor r ^= int64(immI)
        X64_xorqr8  = 0x00F0834800000004u64, // 64bit xor r ^= int64(imm8)
        X64_addlri  = 0xC081400000000003u64, // 32bit add r += immI
        X64_addlr8  = 0x00C0834000000004u64, // 32bit add r += imm8
        X64_andlri  = 0xE081400000000003u64, // 32bit and r &= immI
        X64_andlr8  = 0x00E0834000000004u64, // 32bit and r &= imm8
        X64_orlri   = 0xC881400000000003u64, // 32bit or  r |= immI
        X64_orlr8   = 0x00C8834000000004u64, // 32bit or  r |= imm8
        X64_sublri  = 0xE881400000000003u64, // 32bit sub r -= immI
        X64_sublr8  = 0x00E8834000000004u64, // 32bit sub r -= imm8
        X64_xorlri  = 0xF081400000000003u64, // 32bit xor r ^= immI
        X64_xorlr8  = 0x00F0834000000004u64, // 32bit xor r ^= imm8
        X64_addrr   = 0xC003400000000003u64, // 32bit add r += b
        X64_andqrr  = 0xC023480000000003u64, // 64bit and r &= b
        X64_andrr   = 0xC023400000000003u64, // 32bit and r &= b
        X64_call    = 0x00000000E8000005u64, // near call
        X64_callrax = 0xD0FF000000000002u64, // indirect call to addr in rax (no REX)
		X64_cmovqno = 0xC0410F4800000004u64, // 64bit conditional mov if (no overflow) r = b
        X64_cmovqnae= 0xC0420F4800000004u64, // 64bit conditional mov if (uint <)  r = b
        X64_cmovqnb = 0xC0430F4800000004u64, // 64bit conditional mov if (uint >=) r = b
        X64_cmovqne = 0xC0450F4800000004u64, // 64bit conditional mov if (c)       r = b
        X64_cmovqna = 0xC0460F4800000004u64, // 64bit conditional mov if (uint <=) r = b
        X64_cmovqnbe= 0xC0470F4800000004u64, // 64bit conditional mov if (uint >)  r = b
        X64_cmovqnge= 0xC04C0F4800000004u64, // 64bit conditional mov if (int <)   r = b
        X64_cmovqnl = 0xC04D0F4800000004u64, // 64bit conditional mov if (int >=)  r = b
        X64_cmovqng = 0xC04E0F4800000004u64, // 64bit conditional mov if (int <=)  r = b
        X64_cmovqnle= 0xC04F0F4800000004u64, // 64bit conditional mov if (int >)   r = b
        X64_cmovno  = 0xC0410F4000000004u64, // 32bit conditional mov if (no overflow) r = b
        X64_cmovnae = 0xC0420F4000000004u64, // 32bit conditional mov if (uint <)  r = b
        X64_cmovnb  = 0xC0430F4000000004u64, // 32bit conditional mov if (uint >=) r = b
        X64_cmovne  = 0xC0450F4000000004u64, // 32bit conditional mov if (c)       r = b
        X64_cmovna  = 0xC0460F4000000004u64, // 32bit conditional mov if (uint <=) r = b
        X64_cmovnbe = 0xC0470F4000000004u64, // 32bit conditional mov if (uint >)  r = b
        X64_cmovnge = 0xC04C0F4000000004u64, // 32bit conditional mov if (int <)   r = b
        X64_cmovnl  = 0xC04D0F4000000004u64, // 32bit conditional mov if (int >=)  r = b
        X64_cmovng  = 0xC04E0F4000000004u64, // 32bit conditional mov if (int <=)  r = b
        X64_cmovnle = 0xC04F0F4000000004u64, // 32bit conditional mov if (int >)   r = b
        X64_cmplr   = 0xC03B400000000003u64, // 32bit compare r,b
        X64_cmpqr   = 0xC03B480000000003u64, // 64bit compare r,b
        X64_cmppsr  = 0xC0C20F4000000004u64, // 128bit compare r,b; requires an immediate to specify what kind of comparison
        X64_cmplri  = 0xF881400000000003u64, // 32bit compare r,immI
        X64_cmpqri  = 0xF881480000000003u64, // 64bit compare r,int64(immI)
        X64_cmplr8  = 0x00F8834000000004u64, // 32bit compare r,imm8
        X64_cmpqr8  = 0x00F8834800000004u64, // 64bit compare r,int64(imm8)
        X64_cvtsi2sd= 0xC02A0F40F2000005u64, // convert int32 to double r = (double) b
        X64_cvtsi2ss= 0xC02A0F40F3000005u64, // convert int32 to float r = (float) b
        X64_cvtsq2sd= 0xC02A0F48F2000005u64, // convert int64 to double r = (double) b
        X64_cvtsq2ss= 0xC02A0F48F3000005u64, // convert int64 to float r = (float) b
        X64_cvtss2sd= 0xC05A0F40F3000005u64, // convert float to double r = (double) b
        X64_cvtsd2ss= 0xC05A0F40F2000005u64, // convert double to float r = (float) b
        X64_cvtsd2si= 0xC02D0F40F2000005u64, // convert double to int32 with rounding r = (int32) b
        X64_cvttsd2sq=0xC02C0F48F2000005u64, // convert double to int64 with rounding r = (int64) b
        X64_cvttsd2si=0xC02C0F40F2000005u64, // convert double to int32 r = (int32) b
        X64_cvttss2si=0xC02C0F40F3000005u64, // convert float to int32 r = (int32) b
        X64_divsd   = 0xC05E0F40F2000005u64, // divide scalar double r /= b
        X64_mulsd   = 0xC0590F40F2000005u64, // multiply scalar double r *= b
        X64_addsd   = 0xC0580F40F2000005u64, // add scalar double r += b
        X64_divss   = 0xC05E0F40F3000005u64, // divide scalar single-precision r /= b
        X64_mulss   = 0xC0590F40F3000005u64, // multiply scalar single-precision r *= b
        X64_addss   = 0xC0580F40F3000005u64, // add scalar single-precision r += b
        X64_divps   = 0xC05E0F4000000004u64, // divide float4 vector single-precision r[i] /= b[i]
        X64_mulps   = 0xC0590F4000000004u64, // multiply float4 vector single-precision r[i] *= b[i]
        X64_addps   = 0xC0580F4000000004u64, // add float4 vector single-precision r[i] += b[i]
        X64_idiv    = 0xF8F7400000000003u64, // 32bit signed div (rax = rdx:rax/r, rdx=rdx:rax%r)
        X64_idivq   = 0xF8F7480000000003u64, // 64bit signed div (rax = rdx:rax/r, rdx=rdx:rax%r)
        X64_imul    = 0xC0AF0F4000000004u64, // 32bit signed mul r *= b
        X64_imulq   = 0xC0AF0F4800000004u64, // 64bit signed mul r *= b
        X64_imuli   = 0xC069400000000003u64, // 32bit signed mul r = b * immI
        X64_imulqi  = 0xC069480000000003u64, // 64bit signed mul r = b * immI
        X64_imul8   = 0x00C06B4000000004u64, // 32bit signed mul r = b * imm8
        X64_jmpi    = 0x0000000025FF0006u64, // jump *0(rip)
        X64_jmp     = 0x00000000E9000005u64, // jump near rel32
        X64_jmp8    = 0x00EB000000000002u64, // jump near rel8
        X64_jo      = 0x00000000800F0006u64, // jump near if overflow
        X64_jb      = 0x00000000820F0006u64, // jump near if below (uint <)
        X64_jae     = 0x00000000830F0006u64, // jump near if above or equal (uint >=)
        X64_ja      = 0x00000000870F0006u64, // jump near if above (uint >)
        X64_jbe     = 0x00000000860F0006u64, // jump near if below or equal (uint <=)
        X64_je      = 0x00000000840F0006u64, // near jump if equal
        X64_jl      = 0x000000008C0F0006u64, // jump near if less (int <)
        X64_jge     = 0x000000008D0F0006u64, // jump near if greater or equal (int >=)
        X64_jg      = 0x000000008F0F0006u64, // jump near if greater (int >)
        X64_jle     = 0x000000008E0F0006u64, // jump near if less or equal (int <=)
        X64_jp      = 0x000000008A0F0006u64, // jump near if parity (PF == 1)
        X64_jneg    = 0x0000000001000000u64, // xor with this mask to negate the condition
        X64_jo8     = 0x0070000000000002u64, // jump near if overflow
        X64_jb8     = 0x0072000000000002u64, // jump near if below (uint <)
        X64_jae8    = 0x0073000000000002u64, // jump near if above or equal (uint >=)
        X64_ja8     = 0x0077000000000002u64, // jump near if above (uint >)
        X64_jbe8    = 0x0076000000000002u64, // jump near if below or equal (uint <=)
        X64_je8     = 0x0074000000000002u64, // near jump if equal
        X64_jne8    = 0x0075000000000002u64, // jump near if not equal
        X64_jl8     = 0x007C000000000002u64, // jump near if less (int <)
        X64_jge8    = 0x007D000000000002u64, // jump near if greater or equal (int >=)
        X64_jg8     = 0x007F000000000002u64, // jump near if greater (int >)
        X64_jle8    = 0x007E000000000002u64, // jump near if less or equal (int <=)
        X64_jp8     = 0x007A000000000002u64, // jump near if parity (PF == 1)
        X64_jnp8    = 0x007B000000000002u64, // jump near if not parity (PF == 0)
        X64_jneg8   = 0x0001000000000000u64, // xor with this mask to negate the condition
        X64_leaqrm  = 0x00000000808D4807u64, // 64bit load effective addr reg <- disp32+base
        X64_lealrm  = 0x00000000808D4007u64, // 32bit load effective addr reg <- disp32+base
        X64_learip  = 0x00000000058D4807u64, // 64bit RIP-relative lea. reg <- disp32+rip (modrm = 00rrr101 = 05)
        X64_movlr   = 0xC08B400000000003u64, // 32bit mov r <- b
        X64_movbmr  = 0x0000000080884007u64, // 8bit store r -> [b+d32]
        X64_movsmr  = 0x8089406600000004u64, // 16bit store r -> [b+d32]
        X64_movlmr  = 0x0000000080894007u64, // 32bit store r -> [b+d32]
        X64_movlrm  = 0x00000000808B4007u64, // 32bit load r <- [b+d32]
        X64_movqmr  = 0x0000000080894807u64, // 64bit store gpr -> [b+d32]
        X64_movqspr = 0x0024448948000005u64, // 64bit store gpr -> [rsp+d8] (sib required)
        X64_movqspx = 0x002444110F40F207u64, // 64bit store xmm -> [rsp+d8] (sib required)
        X64_movqr   = 0xC08B480000000003u64, // 64bit mov r <- b
        X64_movqi   = 0xB848000000000002u64, // 64bit mov r <- imm64
        X64_movi    = 0xB840000000000002u64, // 32bit mov r <- immI
        X64_movqi32 = 0xC0C7480000000003u64, // 64bit mov r <- int64(immI)
        X64_movapsr = 0xC0280F4000000004u64, // 128bit mov xmm <- xmm
        X64_unpcklps= 0xC0140F4000000004u64, // 128bit unpack low xmm part
        X64_movqrx  = 0xC07E0F4866000005u64, // 64bit mov b <- xmm-r (reverses the usual r/b order)
        X64_movqxr  = 0xC06E0F4866000005u64, // 64bit mov b -> xmm-r
        X64_movdxr  = 0xC06E0F4066000005u64, // 32bit mov b -> xmm-r
        X64_movqrm  = 0x00000000808B4807u64, // 64bit load r <- [b+d32]
        X64_movsdrr = 0xC0100F40F2000005u64, // 64bit mov xmm-r <- xmm-b (upper 64bits unchanged)
        X64_movupsrm= 0x80100F4000000004u64, // 128bit load xmm-r <- [b+d32] 
        X64_movupspr= 0x2484110F48000005u64, // 128bit float store xmm -> [rsp+d32] (sib required)
        X64_movupsrip=0x05100F4800000004u64, // 128bit load xmm-r <- [RIP+d32] 
        X64_movapsrm= 0x80280F4000000004u64, // 128bit load xmm-r <- [b+d32] 
        X64_movapsrip=0x05280F4800000004u64, // 128bit load xmm-r <- [RIP+d32] 
        X64_movupsmr= 0x80110F4000000004u64, // 128bit store xmm-r -> [b+d32]
        X64_movlhps = 0xC0160F4000000004u64, // 64bit mov r[64:127] <- l[0:63] (the rest unmodified)
        X64_pmovmskb= 0xC0D70F4066000005u64, // move byte mask, r = (first bit from every byte of xmm)
        X64_movsdrm = 0x80100F40F2000005u64, // 64bit load xmm-r <- [b+d32] (upper 64 cleared)
        X64_movsdmr = 0x80110F40F2000005u64, // 64bit store xmm-r -> [b+d32]
        X64_movssrm = 0x80100F40F3000005u64, // 32bit load xmm-r <- [b+d32] (upper 96 cleared)
        X64_movssmr = 0x80110F40F3000005u64, // 32bit store xmm-r -> [b+d32]
        X64_movssspr= 0x2484110F48F30006u64, // 32bit float store xmm -> [rsp+d32] (sib required)
        X64_movsdspr= 0x2484110F48F20006u64, // 64bit float store xmm -> [rsp+d32] (sib required)
        X64_movsxdr = 0xC063480000000003u64, // sign extend i32 to i64 r = (int64)(int32) b
        X64_movzx8  = 0xC0B60F4000000004u64, // zero extend i8 to i64 r = (uint64)(uint8) b
        X64_movzx8m = 0x80B60F4000000004u64, // zero extend i8 load to i32 r <- [b+d32]
        X64_movzx16m= 0x80B70F4000000004u64, // zero extend i16 load to i32 r <- [b+d32]
        X64_movsx8m = 0x80BE0F4000000004u64, // sign extend i8 load to i32 r <- [b+d32]
        X64_movsx16m= 0x80BF0F4000000004u64, // sign extend i16 load to i32 r <- [b+d32]
        X64_neg     = 0xD8F7400000000003u64, // 32bit two's compliment b = -b
        X64_negq    = 0xD8F7480000000003u64, // 64bit two's compliment b = -b
        X64_nop1    = 0x9000000000000001u64, // one byte NOP
        X64_nop2    = 0x9066000000000002u64, // two byte NOP
        X64_nop3    = 0x001F0F0000000003u64, // three byte NOP
        X64_nop4    = 0x00401F0F00000004u64, // four byte NOP
        X64_nop5    = 0x0000441F0F000005u64, // five byte NOP
        X64_nop6    = 0x0000441F0F660006u64, // six byte NOP
        X64_nop7    = 0x00000000801F0F07u64, // seven byte NOP
        X64_not     = 0xD0F7400000000003u64, // 32bit ones compliment b = ~b
        X64_notq    = 0xD0F7480000000003u64, // 64bit ones compliment b = ~b
        X64_orlrr   = 0xC00B400000000003u64, // 32bit or r |= b
        X64_orqrr   = 0xC00B480000000003u64, // 64bit or r |= b
        X64_popr    = 0x5840000000000002u64, // 64bit pop r <- [rsp++]
        X64_pushr   = 0x5040000000000002u64, // 64bit push r -> [--rsp]
        X64_pshufd  = 0xC0700F4066000005u64, // 64bit PSHUFD xmm1,xmm2,imm
        X64_shufpd  = 0xC0C60F4066000005u64, // 64bit SHUFPD xmm1,xmm2,imm
        X64_pxor    = 0xC0EF0F4066000005u64, // 128bit xor xmm-r ^= xmm-b
        X64_ret     = 0xC300000000000001u64, // near return from called procedure
        X64_sete    = 0xC0940F4000000004u64, // set byte if equal (ZF == 1)
        X64_seto    = 0xC0900F4000000004u64, // set byte if overflow (OF == 1)
        X64_setc    = 0xC0920F4000000004u64, // set byte if carry (CF == 1)
        X64_setl    = 0xC09C0F4000000004u64, // set byte if less (int <) (SF != OF)
        X64_setle   = 0xC09E0F4000000004u64, // set byte if less or equal (int <=) (ZF == 1 || SF != OF)
        X64_setg    = 0xC09F0F4000000004u64, // set byte if greater (int >) (ZF == 0 && SF == OF)
        X64_setge   = 0xC09D0F4000000004u64, // set byte if greater or equal (int >=) (SF == OF)
        X64_seta    = 0xC0970F4000000004u64, // set byte if above (uint >) (CF == 0 && ZF == 0)
        X64_setae   = 0xC0930F4000000004u64, // set byte if above or equal (uint >=) (CF == 0)
        X64_setb    = 0xC0920F4000000004u64, // set byte if below (uint <) (CF == 1)
        X64_setbe   = 0xC0960F4000000004u64, // set byte if below or equal (uint <=) (ZF == 1 || CF == 1)
        X64_subsd   = 0xC05C0F40F2000005u64, // subtract scalar double r -= b
        X64_subss   = 0xC05C0F40F3000005u64, // subtract scalar single-precision r -= b
        X64_subps   = 0xC05C0F4000000004u64, // subtract float4 vector single-precision r[i] -= b[i]
        X64_shl     = 0xE0D3400000000003u64, // 32bit left shift r <<= rcx
        X64_shlq    = 0xE0D3480000000003u64, // 64bit left shift r <<= rcx
        X64_shr     = 0xE8D3400000000003u64, // 32bit uint right shift r >>= rcx
        X64_shrq    = 0xE8D3480000000003u64, // 64bit uint right shift r >>= rcx
        X64_sar     = 0xF8D3400000000003u64, // 32bit int right shift r >>= rcx
        X64_sarq    = 0xF8D3480000000003u64, // 64bit int right shift r >>= rcx
        X64_shli    = 0x00E0C14000000004u64, // 32bit left shift r <<= imm8
        X64_shlqi   = 0x00E0C14800000004u64, // 64bit left shift r <<= imm8
        X64_sari    = 0x00F8C14000000004u64, // 32bit int right shift r >>= imm8
        X64_sarqi   = 0x00F8C14800000004u64, // 64bit int right shift r >>= imm8
        X64_shri    = 0x00E8C14000000004u64, // 32bit uint right shift r >>= imm8
        X64_shrqi   = 0x00E8C14800000004u64, // 64bit uint right shift r >>= imm8
        X64_subqrr  = 0xC02B480000000003u64, // 64bit sub r -= b
        X64_subrr   = 0xC02B400000000003u64, // 32bit sub r -= b
        X64_subqri  = 0xE881480000000003u64, // 64bit sub r -= int64(immI)
        X64_subqr8  = 0x00E8834800000004u64, // 64bit sub r -= int64(imm8)
        X64_ucomisd = 0xC02E0F4066000005u64, // unordered compare scalar double
        X64_ucomiss = 0xC02E0F4000000004u64, // unordered compare scalar single-precision float
        X64_xorqrr  = 0xC033480000000003u64, // 64bit xor r &= b
        X64_xorrr   = 0xC033400000000003u64, // 32bit xor r &= b
        X64_xorpd   = 0xC0570F4066000005u64, // 128bit xor xmm (two packed doubles)
        X64_xorps   = 0xC0570F4000000004u64, // 128bit xor xmm (four packed singles), one byte shorter
        X64_xorpsm  = 0x05570F4000000004u64, // 128bit xor xmm, [rip+disp32]
        X64_xorpsa  = 0x2504570F40000005u64, // 128bit xor xmm, [disp32]
        X64_inclmRAX= 0x00FF000000000002u64, // incl (%rax)
        X64_jmpx    = 0xC524ff4000000004u64, // jmp [d32+x*8]
        X64_jmpxb   = 0xC024ff4000000004u64, // jmp [b+x*8]

        X64_movqmi  = 0x80C7480000000003u64, // 32bit signed extended to 64-bit store imm -> qword ptr[b+disp32]
        X64_movlmi  = 0x80C7400000000003u64, // 32bit store imm -> dword ptr[b+disp32]
        X64_movsmi  = 0x80C7406600000004u64, // 16bit store imm -> word ptr[b+disp32]
        X64_movbmi  = 0x80C6400000000003u64, // 8bit store imm -> byte ptr[b+disp32]

        X86_and8r   = 0xC022000000000002u64, // and rl,rh
        X86_sete    = 0xC0940F0000000003u64, // no-rex version of X64_sete
        X86_setnp   = 0xC09B0F0000000003u64  // no-rex set byte if odd parity (ordered fcmp result) (PF == 0)
}