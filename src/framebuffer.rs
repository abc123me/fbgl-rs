extern crate framebuffer;

use framebuffer::Framebuffer;

use crate::*;

pub type Color565 = u16;

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
impl ReprColor for Color565 {
    fn from_rgb(r: u8, g: u8, b: u8) -> Color565 {
        (((b as u16) >> 3) & 0b011111)
            + ((((g as u16) >> 2) & 0b111111) << 5)
            + ((((r as u16) >> 3) & 0b011111) << 11)
    }
    fn to_rgb(&self) -> [u8; 3] {
        [
            ((self & 0b1111_1000_0000_0000) >> 8) as u8, // Red
            ((self & 0b0000_0111_1110_0000) >> 3) as u8, // Green
            ((self & 0b0000_0000_0001_1111) << 3) as u8, // Blue
        ]
    }
}
pub struct DirectFramebufferRenderer<'a, C: FixedFramebufferColor> {
    pub fb: &'a mut Framebuffer,
    _non_generic: C,
}

impl<'a, C: FixedFramebufferColor> DirectFramebufferRenderer<'a, C> {
    pub fn new(fb: &'a mut Framebuffer) -> Result<Self, String> {
        if fb.var_screen_info.bits_per_pixel != C::BITS_T {
            return Err("Bits per pixel must be same as color bits per pixel".to_string());
        }

        Ok(DirectFramebufferRenderer::<C> {
            _non_generic: C::from_rgb(0, 0, 0),
            fb: fb,
        })
    }

    unsafe fn raw_diddy_framebuffer(&self, x: u16, y: u16) -> *const u8 {
        let a = ((y * self.get_width() + x) as usize) * size_of::<C>();
        let b = a + size_of::<C>();
        self.fb.frame[a..b].as_ptr()
    }
}

impl<'a, C: FixedFramebufferColor> GraphicsRenderer for DirectFramebufferRenderer<'a, C> {
    type Color = C;

    fn get_width(&self) -> u16 {
        self.fb.var_screen_info.xres as u16
    }
    fn get_height(&self) -> u16 {
        self.fb.var_screen_info.yres as u16
    }

    fn get_pixel(&self, x: u16, y: u16) -> C {
        let mut col: C = C::new(0, 0, 0);
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
    fn set_pixel(&mut self, col: C, x: u16, y: u16) {
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
