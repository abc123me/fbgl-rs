// ===========================================================
//    ReprColor - Simple trait used to describe a color
// ===========================================================
pub trait ReprColor: Sized + Copy + PartialEq {
	fn from_rgb(r: u8, g: u8, b: u8) -> Self;
	fn to_rgb(&self) -> [u8; 3];

	fn new(r: u8, g: u8, b: u8) -> Self {
		Self::from_rgb(r, g, b)
	}
}

// ===========================================================
//    RGBA - Your more typical 32 bit color (with alpha)
// ===========================================================
#[derive(Clone, Copy, PartialEq)]
pub struct RGBA {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl ReprColor for RGBA {
	fn from_rgb(_r: u8, _g: u8, _b: u8) -> Self {
		RGBA {
			r: _r,
			g: _g,
			b: _b,
			a: 255,
		}
	}
	fn to_rgb(&self) -> [u8; 3] {
		[self.r, self.g, self.b]
	}
}

// ===========================================================
//    Color565 - Simple 16 bit format used by some displays
// ===========================================================

pub type Color565 = u16;

impl ReprColor for Color565 {
	fn from_rgb(r: u8, g: u8, b: u8) -> Color565 {
		(((b as u16) >> 3) & 0b011111)
			+ ((((g as u16) >> 2) & 0b111111) << 5)
			+ ((((r as u16) >> 3) & 0b011111) << 11)
	}
	fn to_rgb(&self) -> [u8; 3] {
		[
			((self & 0b1111_1000_0000_0000) >> 8) as u8, // Red
			((self & 0b0000_0111_1110_0000) >> 3) as u8, // Green
			((self & 0b0000_0000_0001_1111) << 3) as u8, // Blue
		]
	}
}
