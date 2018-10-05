/*!
Defines the x86 instruction struct.
 */

/// Instruction length in bytes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct InstLen {
	/// Total length of the instruction.
	pub total_len: u8,
	/// Length of the operation code.
	pub op_len: u8,
	/// Number of argument bytes.
	pub arg_len: u8,
	/// Number of prefix bytes.
	pub prefix_len: u8,
}
impl InstLen {
	pub const EMPTY: InstLen = InstLen { total_len: 0, op_len: 0, arg_len: 0, prefix_len: 0 };
}
