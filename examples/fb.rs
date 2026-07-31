extern crate framebuffer;
extern crate image;

#[cfg(feature = "img")]
use image::imageops::FilterType;
#[cfg(feature = "img")]
use image::ImageReader;

#[cfg(feature = "img")]
use fbgl::image::ImageOperations;

use framebuffer::{Framebuffer, KdMode};

use fbgl::colors::{Color565, ReprColor};
use fbgl::renderers::fb::DirectFramebufferRenderer;
use fbgl::renderers::heap::HeapBuffer;
use fbgl::renderers::{BufferedRenderer, GraphicsOperations, GraphicsRenderer};

fn print_fb_info(fb: &Framebuffer) {
	let sinf = &fb.var_screen_info;
	let finf = &fb.fix_screen_info;
	println!("Framebuffer variable screen info:");
	println!(" - Bits per pixel: {}", sinf.bits_per_pixel);
	println!(" - Resolution: {}x{}", sinf.xres, sinf.yres);
	println!(
		" - Virtual resolution: {}x{}",
		sinf.xres_virtual, sinf.yres_virtual
	);
	println!(" - Offset: {}x{}", sinf.xoffset, sinf.yoffset);
	println!(
		" - Left/Right margin: {}/{}",
		sinf.left_margin, sinf.right_margin
	);
	println!(
		" - Up/Down margin: {}/{}",
		sinf.upper_margin, sinf.lower_margin
	);
	println!(" - H/V SYNC length: {}/{}", sinf.hsync_len, sinf.vsync_len);
	println!("Framebuffer fixed screen info:");
	println!(
		" - SMEM range: {:#08X} + {:#X}",
		finf.smem_start, finf.smem_len
	);
	println!(
		" - MMIO range: {:#08X} + {:#X}",
		finf.mmio_start, finf.mmio_len
	);
	println!(" - Panstep: {}x{}", finf.xpanstep, finf.ypanstep);
	println!(" - Y-Wrapstep: {}", finf.ywrapstep);
	println!(" - Line length: {}", finf.line_length);
}

fn main() {
	let gfx_mode = Framebuffer::set_kd_mode(KdMode::Graphics);
	if !gfx_mode.is_ok() {
		println!("Failed to set graphics mode on framebuffer!");
	}

	let fb0 = Framebuffer::new("/dev/fb0").unwrap();
	print_fb_info(&fb0);

	let mut gl = HeapBuffer::new(DirectFramebufferRenderer::<Color565>::new(fb0).unwrap());

	println!(
		"Framebuffer fb0 initialized as {}x{}!",
		gl.get_width(),
		gl.get_height()
	);
	let w = gl.get_width();
	let h = gl.get_height();
	let s = h / 2;
	let w2 = w / 2;
	let h2 = h / 2;
	let s2 = s / 2;

	std::thread::sleep(std::time::Duration::from_millis(1000));
	gl.clear(Color565::new(125, 125, 125));
	gl.vline(Color565::new(255, 255, 255), 0);
	gl.vline(Color565::new(255, 255, 255), w - 1);
	gl.hline(Color565::new(255, 255, 255), 0);
	gl.hline(Color565::new(255, 255, 255), h - 1);
	gl.line(Color565::new(255, 0, 0), 0, 0, w - 1, h - 1);
	gl.rect(Color565::new(0, 0, 255), w2 - s2, h2 - s2, s, s);
	gl.rect_outline(Color565::new(255, 255, 255), w2 - s2, h2 - s2, s, s);
	gl.ellipse(Color565::new(255, 255, 0), w2, h2, s, s / 2);
	gl.ellipse_outline(Color565::new(255, 0, 0), w2, h2, s, s / 2);
	gl.push_buffer();

	#[cfg(feature = "img")]
	{
		let img = ImageReader::open("assets/rust-logo.png")
			.unwrap()
			.decode()
			.unwrap()
			.resize_exact(50, 50, FilterType::Triangle)
			.to_rgba8();
		gl.clear(Color565::new(0, 0, 0));
		gl.draw_image_rgba(0, 0, &img);
		gl.push_buffer();

		std::thread::sleep(std::time::Duration::from_millis(100));
		gl.clear(Color565::new(255, 0, 0));
		gl.draw_image_rgba(0, 0, &img);
		gl.push_buffer();

		std::thread::sleep(std::time::Duration::from_millis(100));
		gl.clear(Color565::new(0, 255, 0));
		gl.draw_image_rgba(0, 0, &img);
		gl.push_buffer();

		std::thread::sleep(std::time::Duration::from_millis(100));
		gl.clear(Color565::new(0, 0, 255));
		gl.draw_image_rgba(0, 0, &img);
		gl.push_buffer();
	}

	if gfx_mode.is_ok() {
		let _ = Framebuffer::set_kd_mode(KdMode::Text);
	}
}
