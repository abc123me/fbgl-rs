use devmem::Mapping;

use std::io::{Read, Write, Seek};
use std::fs::File;

use crate::colors::{Color565, ReprColor};
use crate::*;

// AXI DMA framebuffer renderer

pub struct AxiDmaRenderer<C: FixedFramebufferColor> {
	mem_map: Mapping,
	dma_map: Mapping,
	width: u32,
	height: u32,
	pixel_size: u32,
	_non_generic: C,
}

impl<C: FixedFramebufferColor> AxiDmaRenderer<C> {
	pub unsafe fn new(dma_addr: usize, mem_addr: usize, width: u32, height: u32) -> Result<Self, String> {
		let pixel_size = std::mem::size_of::<C>() as u32;
		let mem_len = width * height * pixel_size;
		Ok(AxiDmaRenderer::<C> {
			mem_map: Mapping::new(mem_addr, mem_len),
			dma_map: Mapping::new(dma_addr, 0x1000),
			_non_generic: C::from_rgb(0, 0, 0),
			width, height, pixel_size,
		})
	}
}

impl<C: FixedFramebufferColor> DiddyFbMemory for AxiDmaRenderer<C> {
	unsafe fn raw_diddy_framebuffer(&self, x: u32, y: u32) -> *const u8 {
		let a = ((y as usize) * (self.get_width() as usize) + (x as usize)) * size_of::<C>();
		let b = a + size_of::<C>();
		self.fb.frame[a..b].as_ptr()
	}
}

impl<C: FixedFramebufferColor> GraphicsRenderer for AxiDmaRenderer<C> {
	type Color = C;

	fn get_width(&self) -> u32 {
		self.width
	}
	fn get_height(&self) -> u32 {
		self.height
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

impl GraphicsOperations for AxiDmaRenderer<Color565> {
	fn clear(&mut self, col: Color565) {
		let len = self.fb.frame.len();
		let mut slc: &mut [u16] = bytemuck::cast_slice_mut(&mut self.fb.frame[0..len]);
		slc.fill(col as u16);
	}
}
