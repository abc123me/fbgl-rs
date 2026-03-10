use sdl2::{Sdl, VideoSubsystem, Window};
use sdl2::render::Canvas;

type SdlColor = sdl2::pixels::Color.

impl ReprColor for SdlColor {
	fn from_rgb(r: u8, g: u8, b: u8) -> Self {
		SdlColor::RGB(r, g, b, a)
	}
	fn to_rgb(&self) -> [u8; 3] {
		[self.r, self.g, self.b]
	}
}

pub struct SdlRenderer {
    context: Sdl,
    video: VideoSubsystem,
    window: Window,
	canvas: Canvas
}

impl SdlRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        let context = sdl2::init()?;
        let video = context.video()?;
        let window = video
            .window("FBGL SDL renderer", width, height)
            .position_centered()
            .build()?;
		let canvas = window.into_canvas().build()?;
        Self {
            context,
            video,
            window,
			canvas,
        }
    }
}

impl GraphicsRenderer for SdlFramebufferRenderer {
    type Color = sdl2::pixels::Color;

    fn get_width(&self) -> u32 {
        self.window.size()[0]
    }
    fn get_height(&self) -> u32 {
		self.window.size()[1]
    }

    fn get_pixel(&self, x: u32, y: u32) -> SdlColor {
		SdlColor::RGB(0,0,0)
    }
    fn set_pixel(&mut self, col: SdlColor, x: u32, y: u32) {
		self.canvas.set_draw_color(col);
		self.canvas.draw_point(Point::new(x, y));
    }
}