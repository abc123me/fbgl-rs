use crate::image::ImageOperations;

use image::{GenericImageView, ImageBuffer, Rgba};

use std::time::{Duration, Instant};

#[derive(PartialEq, Copy, Clone)]
pub enum SpriteSheetFormat {
	Vertical,
	Horizontal,
}

pub struct AnimatedSprite {
	current_frame: u32,
	sprites: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
	update_rate: Duration,
	next_update: Instant,
}

impl AnimatedSprite {
	pub fn load_from_image(
		img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
		format: SpriteSheetFormat,
		count: u32,
		sheet_w: u32,
		sheet_h: u32,
		sheet_x: u32,
		sheet_y: u32,
		update_rate: Duration,
	) -> Self {
		let mut x_pos = sheet_x;
		let mut y_pos = sheet_y;
		let mut sprites = Vec::with_capacity(count as usize);

		for _ in 0..count {
			let sprite = img.view(x_pos, y_pos, sheet_w, sheet_h);
			sprites.push(sprite.to_image());
			match format {
				SpriteSheetFormat::Horizontal => x_pos += sheet_w,
				SpriteSheetFormat::Vertical => y_pos += sheet_h,
			}
		}

		Self {
			sprites,
			update_rate,
			next_update: Instant::now(),
			current_frame: 0,
		}
	}

	pub fn advance_frame(&mut self) {
		let next = self.current_frame + 1;
		self.current_frame = next % (self.sprites.len() as u32);
	}

	pub fn schedule_next_frame(&mut self) {
		self.next_update = Instant::now() + self.update_rate;
	}

	pub fn reset_animation(&mut self) {
		self.schedule_next_frame();
		self.current_frame = 0;
	}

	pub fn draw_sprite(&mut self, gl: &mut impl ImageOperations, x: u32, y: u32) {
		gl.draw_image_rgba(x, y, &self.sprites[self.current_frame as usize]);
		if Instant::now() > self.next_update {
			self.advance_frame();
			self.schedule_next_frame();
		}
	}
}
