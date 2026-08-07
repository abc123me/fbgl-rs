extern crate fontdue;

use crate::renderers::GraphicsRenderer;

use fontdue::{Font, LineMetrics};

pub struct TextRenderSettings {
	pub font: Font,
	pub size: u32,
	pub blend: bool,
}

/// Trait for doing generic text rendering operations on a graphics renderer
pub trait TextOperations: GraphicsRenderer {
	/// Draws `txt` with its top-left corner at (x, y), using the font,
	/// size and blending mode described by `opts`. Handles `\n` as a
	/// line break.
	fn text(&mut self, col: Self::Color, x: u32, y: u32, txt: String, opts: TextRenderSettings);

	/// Computes the bounding box (width, height) that `txt` would occupy
	/// if drawn with `opts`, without actually rendering anything
	fn text_size(&self, txt: &str, opts: &TextRenderSettings) -> (u32, u32);
}

impl<T: GraphicsRenderer> TextOperations for T {
	fn text(&mut self, col: Self::Color, x: u32, y: u32, txt: String, opts: TextRenderSettings) {
		let px = opts.size as f32;
		let lm = line_metrics(&opts.font, px);

		let start_x = x as i64;
		let mut pen_x = start_x;
		let mut baseline = y as i64 + lm.ascent.round() as i64;

		let (w, h) = (self.get_width(), self.get_height());

		for ch in txt.chars() {
			if ch == '\n' {
				pen_x = start_x;
				baseline += lm.new_line_size.round() as i64;
				continue;
			}

			let (glyph, bitmap) = opts.font.rasterize(ch, px);

			// Position of the glyph bitmap's top-left corner
			let glyph_x = pen_x + glyph.xmin as i64;
			let glyph_y = baseline - glyph.height as i64 - glyph.ymin as i64;

			for gy in 0..glyph.height {
				let row = gy * glyph.width;
				let py = glyph_y + gy as i64;
				if py < 0 {
					continue;
				}
				let py = py as u32;
				if py >= h {
					break;
				}

				for gx in 0..glyph.width {
					// fontdue gives us an 8 bit coverage/alpha value per pixel
					let alpha = bitmap[row + gx];
					if alpha == 0 {
						continue;
					}

					let px_pos = glyph_x + gx as i64;
					if px_pos < 0 {
						continue;
					}
					let px_pos = px_pos as u32;
					if px_pos >= w {
						continue;
					}

					if opts.blend {
						self.blend_pixel(col, px_pos, py, alpha);
					} else if alpha >= 128 {
						// Non-blending mode simply thresholds coverage
						self.set_pixel(col, px_pos, py);
					}
				}
			}

			pen_x += glyph.advance_width.round() as i64;
		}
	}

	fn text_size(&self, txt: &str, opts: &TextRenderSettings) -> (u32, u32) {
		let px = opts.size as f32;
		let lm = line_metrics(&opts.font, px);

		let mut max_width: f32 = 0.0;
		let mut line_width: f32 = 0.0;
		let mut num_lines: u32 = 1;

		for ch in txt.chars() {
			if ch == '\n' {
				max_width = max_width.max(line_width);
				line_width = 0.0;
				num_lines += 1;
				continue;
			}
			line_width += opts.font.metrics(ch, px).advance_width;
		}
		max_width = max_width.max(line_width);

		let height = lm.ascent.round() as u32
			+ (lm.new_line_size.round() as u32).saturating_mul(num_lines.saturating_sub(1));

		(max_width.round() as u32, height)
	}
}

/// Fetches the font's line metrics for the given pixel size, falling
/// back to sane defaults if the font doesn't provide any
fn line_metrics(font: &Font, px: f32) -> LineMetrics {
	font.horizontal_line_metrics(px).unwrap_or(LineMetrics {
		ascent: px,
		descent: 0.0,
		line_gap: 0.0,
		new_line_size: px,
	})
}
