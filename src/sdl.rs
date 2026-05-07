use crate::*;

use anyhow::{anyhow, Error};

use crate::colors::ReprColor;

use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::{Sdl, VideoSubsystem};

pub type SdlColor = sdl2::pixels::Color;

impl ReprColor for SdlColor {
	fn from_rgb(r: u8, g: u8, b: u8) -> Self {
		SdlColor::RGB(r, g, b)
	}
	fn to_rgb(&self) -> [u8; 3] {
		[self.r, self.g, self.b]
	}
}

pub struct SdlRenderer {
	_context: Sdl,
	_video: VideoSubsystem,
	pub canvas: WindowCanvas,
}

impl SdlRenderer {
	pub fn new(width: u32, height: u32) -> Result<Self, Error> {
		let context =
			sdl2::init().map_err(|err| anyhow!("Failed to initialize SDL context: {}", err))?;
		let video = context
			.video()
			.map_err(|err| anyhow!("Failed to initialize SDL context: {}", err))?;
		let window = video
			.window("FBGL SDL renderer", width, height)
			.position_centered()
			.build()?;
		let canvas = window.into_canvas().build()?;
		Ok(Self {
			_context: context,
			_video: video,
			canvas,
		})
	}
}

impl GraphicsRenderer for SdlRenderer {
	type Color = sdl2::pixels::Color;

	fn get_width(&self) -> u32 {
		self.canvas.output_size().expect("an output size").0
	}
	fn get_height(&self) -> u32 {
		self.canvas.output_size().expect("an output size").1
	}

	fn get_pixel(&self, _x: u32, _y: u32) -> SdlColor {
		SdlColor::RGB(0, 0, 0)
		/* TODO */
	}
	fn set_pixel(&mut self, col: SdlColor, x: u32, y: u32) {
		assert!(x < i32::MAX as u32 && y < i32::MAX as u32);
		self.canvas.set_draw_color(col);
		self.canvas
			.draw_point(Point::new(x as i32, y as i32))
			.expect("draw_point to work");
	}
}
