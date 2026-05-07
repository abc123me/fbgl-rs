use framebuffer::Framebuffer;

use crate::colors::{Color565, ReprColor};
use crate::*;

pub trait FixedFramebufferColor: ReprColor + Sized {
	/// Brief: Total bits
	const BITS_T: u32;
	/// Brief: Red channel bits
	const BITS_R: u32;
	/// Brief: Green channel bits
	const BITS_G: u32;
	/// Brief: Blue channel bits
	const BITS_B: u32;
}
impl FixedFramebufferColor for Color565 {
	const BITS_T: u32 = 16;
	const BITS_R: u32 = 5;
	const BITS_G: u32 = 6;
	const BITS_B: u32 = 5;
}

// Direct framebuffer renderer

pub struct DirectFramebufferRenderer<C: FixedFramebufferColor> {
	fb: Framebuffer,
	_non_generic: C,
}

impl<C: FixedFramebufferColor> DirectFramebufferRenderer<C> {
	pub fn new(fb: Framebuffer) -> Result<Self, String> {
		if fb.var_screen_info.bits_per_pixel != C::BITS_T {
			return Err("Bits per pixel must be same as color bits per pixel".to_string());
		}

		Ok(DirectFramebufferRenderer::<C> {
			_non_generic: C::from_rgb(0, 0, 0),
			fb: fb,
		})
	}
}

impl<C: FixedFramebufferColor> DiddyFbMemory for DirectFramebufferRenderer<C> {
	unsafe fn raw_diddy_framebuffer(&self, x: u32, y: u32) -> *const u8 {
		let a = ((y as usize) * (self.get_width() as usize) + (x as usize)) * size_of::<C>();
		let b = a + size_of::<C>();
		self.fb.frame[a..b].as_ptr()
	}
}

impl<C: FixedFramebufferColor> GraphicsRenderer for DirectFramebufferRenderer<C> {
	type Color = C;

	fn get_width(&self) -> u32 {
		self.fb.var_screen_info.xres
	}
	fn get_height(&self) -> u32 {
		self.fb.var_screen_info.yres
	}

	fn get_pixel(&self, x: u32, y: u32) -> C {
		let col: C = C::new(0, 0, 0);
		// SAFETY - raw_diddy_framebuffer doesn't diddy too much
		unsafe {
			core::ptr::copy_nonoverlapping(
				self.raw_diddy_framebuffer(x, y) as *const u8,
				core::ptr::addr_of!(col) as *mut u8,
				size_of::<C>(),
			);
		}
		col
	}

	fn set_pixel(&mut self, col: C, x: u32, y: u32) {
		// SAFETY - raw_diddy_framebuffer doesn't diddy too much
		unsafe {
			core::ptr::copy_nonoverlapping(
				core::ptr::addr_of!(col) as *const u8,
				self.raw_diddy_framebuffer(x, y) as *mut u8,
				size_of::<C>(),
			);
		}
	}
}

impl GraphicsOperations for DirectFramebufferRenderer<Color565> {
	fn clear(&mut self, col: Color565) {
		let len = self.fb.frame.len();
		let slc: &mut [u16] = bytemuck::cast_slice_mut(&mut self.fb.frame[0..len]);
		slc.fill(col as u16);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_color565() {
		for i in 0_u16..65535_u16 {
			let val: Color565 = i;
			let rgb = val.to_rgb();
			assert_eq!(
				val,
				Color565::from_rgb(rgb[0], rgb[1], rgb[2]),
				"Check color after map from_rgb(to_rgb()) is the same, originally {:#04X} now {:#04X} ({:#02X}, {:#02X}, {:#02X})",
				val, Color565::from_rgb(rgb[0], rgb[1], rgb[2]), rgb[0], rgb[1], rgb[2]
            );
		}
	}
}
