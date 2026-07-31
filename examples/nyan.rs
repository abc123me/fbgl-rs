extern crate image;

use fbgl::colors::ReprColor;
use fbgl::image::ImageOperations;
use fbgl::renderers::sdl::{SdlColor, SdlRenderer};
use fbgl::renderers::{BufferedRenderer, GraphicsOperations, GraphicsRenderer};

use image::{GenericImageView, ImageBuffer, ImageReader, Rgba, SubImage};

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

struct AnimatedSprite {
	current_frame: u32,
	sprites: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
	update_rate: Duration,
	next_update: Instant,
}

impl AnimatedSprite {
	fn load_from_sheet_horizontal(
		img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
		count: u32,
		sheet_w: u32,
		sheet_h: u32,
		sheet_x: u32,
		sheet_y: u32,
		update_rate: Duration,
	) -> Self {
		let mut x_pos = sheet_x;
		let mut sprites = Vec::with_capacity(count as usize);
		for _ in 0..count {
			let sprite = img.view(x_pos, sheet_y, sheet_w, sheet_h);
			sprites.push(sprite.to_image());
			x_pos += sheet_w;
		}

		Self {
			sprites,
			update_rate,
			next_update: Instant::now(),
			current_frame: 0,
		}
	}

	fn advance_frame(&mut self) {
		let next = self.current_frame + 1;
		self.current_frame = next % (self.sprites.len() as u32);
	}

	fn schedule_next_frame(&mut self) {
		self.next_update = Instant::now() + self.update_rate;
	}

	fn reset_animation(&mut self) {
		self.schedule_next_frame();
		self.current_frame = 0;
	}

	fn draw_sprite(&mut self, gl: &mut impl ImageOperations, x: u32, y: u32) {
		gl.draw_image_rgba(x, y, &self.sprites[self.current_frame as usize]);
		if Instant::now() > self.next_update {
			self.advance_frame();
			self.schedule_next_frame();
		}
	}
}

fn main() {
	let mut gl = SdlRenderer::new(500, 500).unwrap();

	let w = gl.get_width();
	let h = gl.get_height();
	let s = h / 2;
	let w2 = w / 2;
	let h2 = h / 2;
	let s2 = s / 2;

	let img = ImageReader::open("assets/nyan-spritesheet.png")
		.unwrap()
		.decode()
		.unwrap()
		.to_rgba8();

	let mut cat = AnimatedSprite::load_from_sheet_horizontal(
		&img,
		6,
		150,
		100,
		0,
		0,
		Duration::from_millis(50),
	);
	let mut rainbow = AnimatedSprite::load_from_sheet_horizontal(
		&img,
		2,
		135,
		100,
		910,
		0,
		Duration::from_millis(150),
	);

	cat.reset_animation();
	rainbow.reset_animation();

	let mut event_pump = gl.context.event_pump().unwrap();
	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				_ => {}
			}
		}

		gl.clear(SdlColor::from_rgb(0, 0, 0));
		rainbow.draw_sprite(&mut gl, 60, 0);
		cat.draw_sprite(&mut gl, 150, 0);
		gl.push_buffer();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
	}
}
