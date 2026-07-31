#[cfg(feature = "fbdev")]
pub mod fb;

#[cfg(feature = "sdl")]
pub mod sdl;

use crate::colors::ReprColor;

/// Trait describing a generic graphics renderer
pub trait GraphicsRenderer {
	type Color: ReprColor;

	fn get_pixel(&self, x: u32, y: u32) -> Self::Color;
	fn get_width(&self) -> u32;
	fn get_height(&self) -> u32;
	fn get_size(&self) -> (u32, u32) {
		(self.get_width(), self.get_height())
	}
	fn get_num_pixels(&self) -> u32 {
		self.get_width() * self.get_height()
	}

	fn set_pixel(&mut self, col: Self::Color, x: u32, y: u32);
	fn set_pixels(&mut self, pixels: &Vec<Self::Color>, x1: u32, y1: u32, x2: u32, y2: u32) {
		let w = self.get_width();
		assert!(
			pixels.len() == self.get_num_pixels() as usize,
			"Given incorrect amount of pixels, given {}, got {}!",
			pixels.len(),
			self.get_num_pixels() as usize,
		);
		for y in y1..y2 {
			let p = y * w;
			for x in x1..x2 {
				self.set_pixel(pixels[(x + p) as usize], x, y);
			}
		}
	}
	fn blend_pixel(&mut self, col: Self::Color, x: u32, y: u32, alpha: u8) {
		let (old, new) = (self.get_pixel(x, y).to_rgb(), col.to_rgb());
		let alpha16 = alpha as u16;
		let alpha_orig = 256_u16 - alpha16;
		let r = ((old[0] as u16 * alpha_orig) + (new[0] as u16 * alpha16)) / 256;
		let g = ((old[1] as u16 * alpha_orig) + (new[1] as u16 * alpha16)) / 256;
		let b = ((old[2] as u16 * alpha_orig) + (new[2] as u16 * alpha16)) / 256;
		self.set_pixel(Self::Color::from_rgb(r as u8, g as u8, b as u8), x, y);
	}
}

/// Trait describing a generic buffered renderer
pub trait BufferedRenderer {
	fn push_buffer(&mut self);
}

/// Trait describing graphical operations that are typical
/// of something implementing the graphics renderer trait
pub trait GraphicsOperations: GraphicsRenderer {
	/* For branch prediction reasons, all of these are assumed to
	 * have appropriate input values, and if not you're fucked */
	fn clear(&mut self, col: Self::Color) {
		/* chose to iterate over y here since most displays
		 * draw on a row-by-row / scan line basis */
		for p in 0..self.get_height() {
			self.hline(col, p);
		}
	}
	fn rect(&mut self, col: Self::Color, x: u32, y: u32, w: u32, h: u32) {
		for xi in 0..w {
			for yi in 0..h {
				self.set_pixel(col, x + xi, y + yi);
			}
		}
	}
	fn rect_outline(&mut self, col: Self::Color, x: u32, y: u32, w: u32, h: u32) {
		self.vline_bounded(col, x, y, y + h);
		self.vline_bounded(col, x + w, y, y + h);
		self.hline_bounded(col, y, x, x + w);
		self.hline_bounded(col, y + h, x, x + w);
		self.set_pixel(col, x + w, y + h);
	}
	fn circle(&mut self, col: Self::Color, x: u32, y: u32, s: u32) {
		self.ellipse(col, x, y, s, s)
	}
	fn circle_outline(&mut self, col: Self::Color, x: u32, y: u32, s: u32) {
		self.ellipse_outline(col, x, y, s, s)
	}
	fn ellipse(&mut self, col: Self::Color, x: u32, y: u32, w: u32, h: u32) {
		let hh = h * h;
		let ww = w * w;
		let (mut x0, mut dx, hhww) = (w, 0, hh * ww);

		for yp in 0..h {
			let mut x1 = x0 - (dx - 1);
			while x1 > 0 {
				if x1 * x1 * hh + yp * yp * ww <= hhww {
					break;
				}
				x1 -= 1;
			}

			// approximate the slope
			dx = x0 - x1;
			x0 = x1;

			for xp in 0..x0 {
				self.set_pixel(col, x + xp, y + yp);
				self.set_pixel(col, x + xp, y - yp);
				self.set_pixel(col, x - xp, y + yp);
				self.set_pixel(col, x - xp, y - yp);
			}
		}
	}
	fn ellipse_outline(&mut self, _col: Self::Color, _x: u32, _y: u32, _w: u32, _h: u32) {
		//todo
	}
	fn line(&mut self, col: Self::Color, x1: u32, y1: u32, x2: u32, y2: u32) {
		let dx = (x2 as i32) - (x1 as i32);
		let dy = (y2 as i32) - (y1 as i32);
		// Bitwise OR here is faster then max
		// and has no branch prediction impacts
		let ds = dx.abs() | dy.abs();
		for i in 0..ds {
			let xp = x1 as i32 + (dx * i) / ds;
			let yp = y1 as i32 + (dy * i) / ds;
			self.set_pixel(col, xp as u32, yp as u32);
		}
	}
	fn hline_bounded(&mut self, col: Self::Color, yp: u32, x1: u32, x2: u32) {
		assert!(x2 > x1, "x2 must be greater then x1 for hline_bounded!");
		for xp in x1..x2 {
			self.set_pixel(col, xp, yp);
		}
	}
	fn vline_bounded(&mut self, col: Self::Color, xp: u32, y1: u32, y2: u32) {
		assert!(y2 > y1, "y2 must be greater then y1 for vline_bounded!");
		for yp in y1..y2 {
			self.set_pixel(col, xp, yp);
		}
	}
	fn hline(&mut self, col: Self::Color, p: u32) {
		self.hline_bounded(col, p, 0, self.get_width());
	}
	fn vline(&mut self, col: Self::Color, p: u32) {
		self.vline_bounded(col, p, 0, self.get_height());
	}
}

/// Trait for directly accessing framebuffer memory
pub(crate) trait DiddyFbMemory {
	unsafe fn raw_diddy_framebuffer(&self, x: u32, y: u32) -> *const u8;
}
