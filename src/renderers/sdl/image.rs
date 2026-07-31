extern crate image;

use super::SdlRenderer;

use image::{GenericImageView, Pixel, Rgb, Rgba};

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

impl crate::image::ImageOperations for SdlRenderer {
	fn draw_image_rgb(&mut self, px: u32, py: u32, img: &impl GenericImageView<Pixel = Rgb<u8>>) {
		let (width, height) = img.dimensions();

		// Build a tightly-packed RGBA8 buffer from the GenericImageView
		let mut rgb_bytes: Vec<u8> = Vec::with_capacity((width * height * 3) as usize);
		for y in 0..height {
			for x in 0..width {
				let px = img.get_pixel(x, y).to_rgb();
				rgb_bytes.extend_from_slice(&px.0);
			}
		}

		// Wrap the raw bytes in an SDL Surface (borrows rgba_bytes, so keep it alive)
		let surface = Surface::from_data(
			&mut rgb_bytes,
			width,
			height,
			width * 3,
			PixelFormatEnum::RGB24,
		)
		.expect("able to create a surface");

		let texture = self
			.texture_creator
			.create_texture_from_surface(&surface)
			.map_err(|e| e.to_string())
			.expect("able to create a texture");

		self.canvas
			.copy(
				&texture,
				None,
				Some(Rect::new(px as i32, py as i32, width, height)),
			)
			.expect("able to copy texture to canvas");
	}
	fn draw_image_rgba(&mut self, px: u32, py: u32, img: &impl GenericImageView<Pixel = Rgba<u8>>) {
		let (width, height) = img.dimensions();

		// Build a tightly-packed RGBA8 buffer from the GenericImageView
		let mut rgba_bytes: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
		for y in 0..height {
			for x in 0..width {
				let px = img.get_pixel(x, y).to_rgba();
				rgba_bytes.extend_from_slice(&px.0); // [r, g, b, a]
			}
		}

		let pitch = width * 4;

		// Wrap the raw bytes in an SDL Surface (borrows rgba_bytes, so keep it alive)
		let surface = Surface::from_data(
			&mut rgba_bytes,
			width,
			height,
			pitch,
			PixelFormatEnum::RGBA32,
		)
		.expect("able to create a surface");

		let texture = self
			.texture_creator
			.create_texture_from_surface(&surface)
			.map_err(|e| e.to_string())
			.expect("able to create a texture");

		self.canvas
			.copy(
				&texture,
				None,
				Some(Rect::new(px as i32, py as i32, width, height)),
			)
			.expect("able to copy texture to canvas");
	}
}
