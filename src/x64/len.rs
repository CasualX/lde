/*!
Opcode length tables (X64).
 */

#![allow(unused)]

/// Length classification.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum LenClass {
	Invalid,     // Invalid opcode
	Prefix,      // Invalid opcode, byte is a prefix
	TwoByte,     // Two byte opcodes 0f
	Valid,       // Valid opcode, no mod/rm, no immediate
	VAbs,        // Valid opcode with absolute mem arg
	VImm8,       // Valid opcode with imm8 arg
	VImm16,      // Valid opcode with imm16 arg
	VImm32,      // Valid opcode with imm32 arg (affected by address size override)
	VModRM,      // Valid opcode with mod/rm arg
	VModRMImm8,  // Valid opcode with mod/rm and imm8 args
	VModRMImm32, // Valid opcode with mod/rm and imm32 args
}
use self::LenClass::*;

pub static LEN_X64: [LenClass; 0xc0] = [
	VModRM, // 00: ADD Eb, Gb
	VModRM, // 01: ADD Ev, Gv
	VModRM, // 02: ADD Gb, Eb
	VModRM, // 03: ADD Gv, Ev
	VImm8,  // 04: ADD AL, Ib
	VImm32, // 05: ADD rAX, Iz
	Invalid,
	Invalid,

	VModRM, // 08: OR Eb, Gb
	VModRM, // 09: OR Ev, Gv
	VModRM, // 0a: OR Gb, Eb
	VModRM, // 0b: OR Gv, Ev
	VImm8,  // 0c: OR AL, Ib
	VImm32, // 0d: OR rAX, Iz
	Invalid,
	TwoByte,

	VModRM, // 10: ADC Eb, Gb
	VModRM, // 11: ADC Ev, Gv
	VModRM, // 12: ADC Gb, Eb
	VModRM, // 13: ADC Gv, Ev
	VImm8,  // 14: ADC AL, Ib
	VImm32, // 15: ADC rAX, Iz
	Invalid,
	Invalid,

	VModRM, // 18: SBB Eb, Gb
	VModRM, // 19: SBB Ev, Gv
	VModRM, // 1a: SBB Gb, Eb
	VModRM, // 1b: SBB Gv, Ev
	VImm8,  // 1c: SBB AL, Ib
	VImm32, // 1d: SBB rAX, Iz
	Invalid,
	Invalid,

	VModRM, // 20: AND Eb, Gb
	VModRM, // 21: AND Ev, Gv
	VModRM, // 22: AND Gb, Eb
	VModRM, // 23: AND Gv, Ev
	VImm8,  // 24: AND AL, Ib
	VImm32, // 25: AND rAX, Iz
	Prefix, // 26: Null prefix
	Invalid,

	VModRM, // 28: SUB Eb, Gb
	VModRM, // 29: SUB Ev, Gv
	VModRM, // 2a: SUB Gb, Eb
	VModRM, // 2b: SUB Gv, Ev
	VImm8,  // 2c: SUB AL, Ib
	VImm32, // 2d: SUB rAX, Iz
	Prefix, // 2e: Null prefix
	Invalid,

	VModRM, // 30: XOR Eb, Gb
	VModRM, // 31: XOR Ev, Gv
	VModRM, // 32: XOR Gb, Eb
	VModRM, // 33: XOR Gv, Ev
	VImm8,  // 34: XOR AL, Ib
	VImm32, // 35: XOR rAX, Iz
	Prefix, // 36: Null prefix
	Invalid,

	VModRM, // 38: CMP Eb, Gb
	VModRM, // 39: CMP Ev, Gv
	VModRM, // 3a: CMP Gb, Eb
	VModRM, // 3b: CMP Gv, Ev
	VImm8,  // 3c: CMP AL, Ib
	VImm32, // 3d: CMP rAX, Iz
	Prefix, // 3e: Null prefix
	Invalid,

	// REX prefixes
	Prefix, Prefix, Prefix, Prefix, Prefix, Prefix, Prefix, Prefix,
	Prefix, Prefix, Prefix, Prefix, Prefix, Prefix, Prefix, Prefix,

	Valid, Valid, Valid, Valid, Valid, Valid, Valid, Valid, // PUSH Zv
	Valid, Valid, Valid, Valid, Valid, Valid, Valid, Valid, // POP Zv

	Invalid,
	Invalid,
	Invalid,
	VModRM,      // 63: MOVSXD Gv, Ev
	Prefix,      // 64: FS prefix
	Prefix,      // 65: GS prefix
	Prefix,      // 66: Operand size prefix
	Prefix,      // 67: Address size prefix
	VImm32,      // 68: PUSH Iz
	VModRMImm32, // 69: IMUL Gv, Ev, Iz
	VImm8,       // 6a: PUSH Ib
	VModRMImm8,  // 6b: IMUL Gv, Ev, Ib
	Valid,       // 6c: INS B Yb, DX
	Valid,       // 6d: INS W/D Yz,DX
	Valid,       // 6e: OUTS B DX, Xb
	Valid,       // 6f: OUTS W/D DX, Xz

	// JCC Jb
	VImm8, VImm8, VImm8, VImm8, VImm8, VImm8, VImm8, VImm8,
	VImm8, VImm8, VImm8, VImm8, VImm8, VImm8, VImm8, VImm8,

	VModRMImm8,  // 80: Group1 Eb, Ib
	VModRMImm32, // 81: Group1 Ev, Iz
	Invalid,
	VModRMImm8, // 83: Group1 Ev, Ib
	VModRM,     // 84: TEST Eb, Gb
	VModRM,     // 85: TEST Ev, Gv
	VModRM,     // 86: XCHG Eb, Gb
	VModRM,     // 87: XCHG Ev, Gv
	VModRM,     // 88: MOV Eb, Gb
	VModRM,     // 89: MOV Ev, Gv
	VModRM,     // 8a: MOV Gb, Eb
	VModRM,     // 8b: MOV Gv, Ev
	VModRM,     // 8c: MOV Ev, Sw
	VModRM,     // 8d: LEA Gv, M
	VModRM,     // 8e: MOV Sw, Ew
	VModRM,     // 8f: POP Ev

	// 90: XCHG Zv, rAX
	Valid, Valid, Valid, Valid, Valid, Valid, Valid, Valid,

	Valid, // 98: CBW/CWDE/CDQE
	Valid, // 99: CWD/CDQ/CQO
	Invalid,
	Valid, // 9b: FWAIT
	Valid, // 9c: PUSHF
	Valid, // 9d: POPF
	Valid, // 9e: SAHF
	Valid, // 9f: LAHF

	VAbs,  // a0: MOV AL, Ob
	VAbs,  // a1: MOV rAX, Ov
	VAbs,  // a2: MOV Ob, AL
	VAbs,  // a3: MOV Ob, rAX
	Valid, // a4: MOVS/B
	Valid, // a5: MOVS/W/D/Q
	Valid, // a6: CMPS/B
	Valid, // a7: CMPS/W/D/Q

	VImm8,  // a8: TEST AL, Ib
	VImm32, // a9: TEXT rAX, Iz
	Valid,  // aa: STOS/B
	Valid,  // ab: STOS/W/D/Q
	Valid,  // ac: LODS/B
	Valid,  // ad: LODS/W/D/Q
	Valid,  // ae: SCAS/B
	Valid,  // af: SCAS/W/D/Q

	VImm8, VImm8, VImm8, VImm8, VImm8, VImm8, VImm8, VImm8,         // b0: MOV Zb, Ib
	VImm32, VImm32, VImm32, VImm32, VImm32, VImm32, VImm32, VImm32, // b8: MOV Zv, Iz

	//VModRMImm8, // Group2 Eb, Ib
	//VModRMImm8, // Group2 Ev, Ib
	//VImm16, // c2: RETN Iw
	//Valid, // c3: RETN
	//Invalid, // c4: VEX+2byte prefix
	//Invalid, // c5: VEX+1byte prefix
];
