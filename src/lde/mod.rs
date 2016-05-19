pub mod x64;
pub mod x86;

/* For copy pasting purposes.
static TABLE_EMPTY: [u32; 8] = [
	/* 0 1 2 3 4 5 6 7 8 9 A B C D E F 0 1 2 3 4 5 6 7 8 9 A B C D E F */
	0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0,// 0
	0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0,// 2
	0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0,// 4
	0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0,// 6
	0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0,// 8
	0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0,// A
	0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0,// C
	0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0,// E
];
*/

// Convenience for checking if byte is contained within.
trait Contains {
	fn has(&self, val: u8) -> bool;
}
impl Contains for [u32; 8] {
	#[inline(always)]
	fn has(&self, val: u8) -> bool {
		(self[((val >> 5) & 7) as usize] & (0x80000000 >> (val & 0x1F))) != 0
	}
}
impl Contains for [u32; 2] {
	#[inline(always)]
	fn has(&self, val: u8) -> bool {
		if val < 0x40 {
			(self[((val >> 5) & 7) as usize] & (0x80000000 >> (val & 0x1F))) != 0
		}
		else {
			false
		}
	}
}
use ::core::ops::Range;
impl Contains for Range<u8> {
	#[inline(always)]
	fn has(&self, val: u8) -> bool {
		val.wrapping_sub(self.start) < self.end.wrapping_sub(self.start)
	}
}
