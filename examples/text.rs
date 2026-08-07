extern crate image;

use fbgl::colors::ReprColor;
use fbgl::renderers::sdl::{SdlColor, SdlRenderer};
use fbgl::renderers::{BufferedRenderer, GraphicsOperations};
use fbgl::text::{TextOperations, TextRenderSettings};

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
	let bts = include_bytes!("/usr/share/fonts/noto/NotoSans-Bold.ttf") as &[u8];
	let mut gl = SdlRenderer::new(500, 500).unwrap();

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
		let trs = TextRenderSettings {
			font: fontdue::Font::from_bytes(
				bts,
				fontdue::FontSettings {
					scale: 32.0,
					..fontdue::FontSettings::default()
				},
			)
			.unwrap(),
			size: 32,
			blend: false,
		};
		gl.text(
			SdlColor::from_rgb(255, 255, 255),
			0,
			0,
			"Hello world!".to_string(),
			trs,
		);
		gl.push_buffer();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
	}
}
