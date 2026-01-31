use fbgl::framebuffer::*;
use fbgl::*;

extern crate framebuffer;
extern crate image;

#[cfg(feature = "img")]
use image::imageops::FilterType;
#[cfg(feature = "img")]
use image::ImageReader;

#[cfg(feature = "img")]
use fbgl::image::ImageOperations;

use framebuffer::{Framebuffer, KdMode};

fn main() {
    let mut fb = Framebuffer::new("/dev/fb0").unwrap();
    let gfx_mode = Framebuffer::set_kd_mode(KdMode::Graphics);
    if !gfx_mode.is_ok() {
        println!("Failed to set graphics mode on framebuffer!");
    }

    //let mut gl = BufferedRenderer::<DirectFramebufferRenderer<Color565>>::new(
    //    DirectFramebufferRenderer::<Color565>::new(&mut fb).unwrap(),
    //);
    let mut gl = DirectFramebufferRenderer::<Color565>::new(&mut fb).unwrap();

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

    gl.clear(Color565::new(125, 125, 125));
    //gl.push_buffer();

    //std::thread::sleep(std::time::Duration::from_millis(1000));
    gl.clear(Color565::new(0, 0, 0));
    gl.vline(Color565::new(255, 255, 255), 0);
    gl.vline(Color565::new(255, 255, 255), w - 1);
    gl.hline(Color565::new(255, 255, 255), 0);
    gl.hline(Color565::new(255, 255, 255), h - 1);
    gl.line(Color565::new(255, 0, 0), 0, 0, w - 1, h - 1);
    gl.rect(Color565::new(0, 0, 255), w2 - s2, h2 - s2, s, s);
    gl.rect_outline(Color565::new(255, 255, 255), w2 - s2, h2 - s2, s, s);
    gl.ellipse(Color565::new(255, 255, 0), w2, h2, s, s / 2);
    gl.ellipse_outline(Color565::new(255, 0, 0), w2, h2, s, s / 2);
    //gl.push_buffer();

    #[cfg(feature = "img")]
    {
        let img = ImageReader::open("rust-logo.png")
            .unwrap()
            .decode()
            .unwrap()
            .resize_exact(50, 50, FilterType::Triangle)
            .to_rgba8();
        gl.clear(Color565::new(0, 0, 0));
        gl.draw_image_rgba(0, 0, &img);
        //gl.push_buffer();
        std::thread::sleep(std::time::Duration::from_millis(100));
        gl.clear(Color565::new(255, 0, 0));
        gl.draw_image_rgba(0, 0, &img);
        //gl.push_buffer();
        std::thread::sleep(std::time::Duration::from_millis(100));
        gl.clear(Color565::new(0, 255, 0));
        gl.draw_image_rgba(0, 0, &img);
        //gl.push_buffer();
        std::thread::sleep(std::time::Duration::from_millis(100));
        gl.clear(Color565::new(0, 0, 255));
        gl.draw_image_rgba(0, 0, &img);
        //gl.push_buffer();
    }

    if gfx_mode.is_ok() {
        let _ = Framebuffer::set_kd_mode(KdMode::Text);
    }
}
