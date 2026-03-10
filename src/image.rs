extern crate image;

use crate::*;

use image::{GenericImageView, Pixel, Rgb, Rgba};

/// Brief - Trait for doing generic image operations on a graphics renderer
pub trait ImageOperations: GraphicsRenderer {
	/// Brief - Method for drawing an image without alpha, faster
	fn draw_image_rgb(&mut self, x: u32, y: u32, img: &impl GenericImageView<Pixel = Rgb<u8>>);
	/// Brief - Method for drawing an image with an alpha channel, slower
	fn draw_image_rgba(&mut self, x: u32, y: u32, img: &impl GenericImageView<Pixel = Rgba<u8>>);
}

impl<T: GraphicsRenderer> ImageOperations for T {
	fn draw_image_rgb(&mut self, x: u32, y: u32, img: &impl GenericImageView<Pixel = Rgb<u8>>) {
		for (x, y, p) in img.pixels() {
			let c = p.to_rgb();
			self.set_pixel(T::Color::new(c[0], c[1], c[2]), x, y);
		}
	}
	fn draw_image_rgba(&mut self, x: u32, y: u32, img: &impl GenericImageView<Pixel = Rgba<u8>>) {
		for (x, y, p) in img.pixels() {
			let c = p.to_rgba();
			self.blend_pixel(T::Color::new(c[0], c[1], c[2]), x, y, c[3]);
		}
	}
}
