extern crate image;

use fbgl::colors::ReprColor;
use fbgl::image::sprite::{AnimatedSprite, SpriteSheetFormat};
use fbgl::renderers::sdl::{SdlColor, SdlRenderer};
use fbgl::renderers::{BufferedRenderer, GraphicsOperations, GraphicsRenderer};

use image::ImageReader;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

	let mut cat = AnimatedSprite::load_from_image(
		&img,
		SpriteSheetFormat::Horizontal,
		6,
		150,
		100,
		0,
		0,
		Duration::from_millis(50),
	);
	let mut rainbow = AnimatedSprite::load_from_image(
		&img,
		SpriteSheetFormat::Horizontal,
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
