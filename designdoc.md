
# Raw Prefixes (u16)

26, 2E, 36, 3E,
40-4F,
64, 65, 66, 67,
9B, F0, F2, F3,

ES, CS, NTAKEN, SS, DS, TAKEN, REX.W, REX.R, REX.X, REX.B, FS, ALTER, GS,
OPERAND_SIZE, PRECISION_SIZE, ADDRESS_SIZE, WAIT, LOCK, REPNZ, REPNE, REP, REPZ, REPE
SCALAR_DOUBLE_PRECISION, SCALAR_SINGLE_PRECISION,

Requires 16 bits to represent.

```rust
pub struct RawPrefix(pub u32);
impl RawPrefix {
	pub const ES: RawPrefix     = RawPrefix(0x0001); // 26
	pub const CS: RawPrefix     = RawPrefix(0x0002); // 2E
	pub const SS: RawPrefix     = RawPrefix(0x0004); // 36
	pub const DS: RawPrefix     = RawPrefix(0x0008); // 3E
	pub const NTAKEN: RawPrefix = RawPrefix(0x0002); // 2E
	pub const TAKEN: RawPrefix  = RawPrefix(0x0008); // 3E

	pub const FS: RawPrefix     = RawPrefix(0x0010); // 64
	pub const GS: RawPrefix     = RawPrefix(0x0020); // 65
	pub const ALTER: RawPrefix  = RawPrefix(0x0010); // 64
	pub const WAIT: RawPrefix   = RawPrefix(0x0040); // 9B
	pub const LOCK: RawPrefix   = RawPrefix(0x0080); // F0

	pub const OPERAND_SIZE: RawPrefix   = RawPrefix(0x0100); // 66
	pub const PRECISION_SIZE: RawPrefix = RawPrefix(0x0100); // 66
	pub const ADDRESS_SIZE: RawPrefix   = RawPrefix(0x0200); // 67

	pub const REP: RawPrefix   = RawPrefix(0x0C00);
	pub const REPNZ: RawPrefix = RawPrefix(0x0400); // F2
	pub const REPNE: RawPrefix = RawPrefix(0x0400); // F2
	pub const REPN: RawPrefix  = RawPrefix(0x0800); // F3
	pub const REPZ: RawPrefix  = RawPrefix(0x0800); // F3

	pub const SCALAR_DOUBLE_PRECISION: RawPrefix = RawPrefix(0x0400); // F2
	pub const SCALAR_SINGLE_PRECISION: RawPrefix = RawPrefix(0x0800); // F3

	pub const REX: RawPrefix     = RawPrefix(0x10000);
	pub const REX_B: RawPrefix   = RawPrefix(0x11000);
	pub const REX_X: RawPrefix   = RawPrefix(0x12000);
	pub const REX_R: RawPrefix   = RawPrefix(0x14000);
	pub const REX_W: RawPrefix   = RawPrefix(0x18000);

	pub fn parse(iter: &mut &[u8]) -> RawPrefix {
		let mut bytes = *iter;
		let mut raw_prefix = 0;
		while let Some(byte) = bytes.get(0) {
			match &byte {
				0x26 => raw_prefix |= 0x0001,
				0x2E => raw_prefix |= 0x0002,
				0x36 => raw_prefix |= 0x0004,
				0x3E => raw_prefix |= 0x0008,

				0x64 => raw_prefix |= 0x0010,
				0x65 => raw_prefix |= 0x0020,
				0x9B => raw_prefix |= 0x0040,
				0xF0 => raw_prefix |= 0x0080,

				0x66 => raw_prefix |= 0x0100,
				0x67 => raw_prefix |= 0x0200,
				0xF2 => raw_prefix |= 0x0400,
				0xF3 => raw_prefix |= 0x0800,

				0x40..0x50 => raw_prefix |= 0x10000 | (byte & 0xF0) as u32 << 12,

				_ => break,
			}
		}
		*iter = bytes;
		RawPrefix(raw_prefix)
	}
}
```

# Opcode (u32)

0x0f

0x000fab
0x000001
0x0f3800

  5f  38  20
[ of, po, so ]

Efficient encoding for tree-based lookup tables.

```rust
fn parse_opcode(&mut &[u8]) -> Result<u32>;
```

# ModRM

mod_rm: u8,
sib: u8,
displacement: u32,
immediate: u32,

    fn parse_modrm(&mut &[u8]) -> Result<ModRM>;
