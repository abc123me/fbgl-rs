extern crate image;

pub mod sprite;

use crate::colors::ReprColor;
use crate::renderers::GraphicsRenderer;

use image::{GenericImageView, Pixel, Rgb, Rgba};

/// Trait for doing generic image operations on a graphics renderer
pub trait ImageOperations: GraphicsRenderer {
	/// Method for drawing an image without alpha, faster
	fn draw_image_rgb(&mut self, px: u32, py: u32, img: &impl GenericImageView<Pixel = Rgb<u8>>) {
		for (x, y, p) in img.pixels() {
			let c = p.to_rgb();
			self.set_pixel(Self::Color::new(c[0], c[1], c[2]), px + x, py + y);
		}
	}
	/// Method for drawing an image with an alpha channel, slower
	fn draw_image_rgba(&mut self, px: u32, py: u32, img: &impl GenericImageView<Pixel = Rgba<u8>>) {
		for (x, y, p) in img.pixels() {
			let c = p.to_rgba();
			self.blend_pixel(Self::Color::new(c[0], c[1], c[2]), px + x, y + py, c[3]);
		}
	}
}
