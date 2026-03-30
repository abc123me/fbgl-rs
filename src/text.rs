extern crate fontdue;

use crate::*;

use fontdue::Font;

pub struct TextRenderSettings {
	pub font: Font,
	pub size: u32,
	pub blend: bool,
}

pub trait TextOperations: GraphicsRenderer {
	fn text(&mut self, col: Self::Color, x: u32, y: u32, txt: String, opts: TextRenderSettings);
}

impl<T: GraphicsRenderer> TextOperations for T {
	fn text(
		&mut self,
		_col: Self::Color,
		_x: u32,
		_y: u32,
		_txt: String,
		_opts: TextRenderSettings,
	) {
		// TODO
	}
}
