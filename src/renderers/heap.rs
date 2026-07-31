use crate::renderers::{GraphicsOperations, GraphicsRenderer};

#[cfg(feature = "img")]
pub mod image;

/// Heap buffer for a generic renderer
pub struct HeapBuffer<T: GraphicsRenderer> {
	pub base: T,
	buffer_width: u32,
	buffer_height: u32,
	buffer_x_min: u32,
	buffer_y_min: u32,
	buffer_x_max: u32,
	buffer_y_max: u32,
	buffer: Vec<T::Color>,
}

impl<T: GraphicsRenderer> HeapBuffer<T> {
	pub fn new(base_renderer: T) -> Self {
		let mut out = HeapBuffer::<T> {
			base: base_renderer,
			buffer_width: 0,
			buffer_height: 0, /* these are set by resize_to_base */
			buffer_x_min: 0,
			buffer_y_min: 0,
			buffer_x_max: 0,
			buffer_y_max: 0,
			buffer: Vec::new(),
		};
		out.resize_to_base();
		out
	}

	pub fn resize_to_base(&mut self) {
		self.buffer_width = self.base.get_width();
		self.buffer_height = self.base.get_height();
		self.buffer_x_max = self.buffer_width;
		self.buffer_y_max = self.buffer_height;
		(self.buffer_x_min, self.buffer_y_min) = (0, 0);

		self.buffer = Vec::<T::Color>::with_capacity(self.base.get_num_pixels() as usize);
		for _i in 0..self.base.get_num_pixels() {
			self.buffer.push(T::Color::new(0, 0, 0));
		}

		println!(
			"HeapBuffer size set to {} x {} ({} pixels)",
			self.buffer_width,
			self.buffer_height,
			self.base.get_num_pixels()
		);
	}
}

impl<T: GraphicsRenderer> BufferedRenderer for HeapBuffer<T> {
	fn push_buffer(&mut self) {
		self.base.set_pixels(
			&self.buffer,
			self.buffer_x_min,
			self.buffer_y_min,
			self.buffer_x_max,
			self.buffer_y_max,
		);
		(self.buffer_x_max, self.buffer_y_max) = (0, 0);
		self.buffer_x_min = self.buffer_width;
		self.buffer_y_min = self.buffer_height;
	}
}

impl<T: GraphicsRenderer> GraphicsRenderer for HeapBuffer<T> {
	type Color = T::Color;

	fn get_pixel(&self, x: u32, y: u32) -> T::Color {
		let w = self.get_width();
		self.buffer[(x + y * w) as usize]
	}
	fn get_width(&self) -> u32 {
		self.buffer_width
	}
	fn get_height(&self) -> u32 {
		self.buffer_height
	}

	fn set_pixel(&mut self, col: T::Color, x: u32, y: u32) {
		let w = self.get_width();
		if self.buffer[(x + y * w) as usize] != col {
			self.buffer_x_min = self.buffer_x_min.min(x);
			self.buffer_y_min = self.buffer_y_min.min(y);
			self.buffer_x_max = self.buffer_x_max.max(x + 1);
			self.buffer_y_max = self.buffer_y_max.max(y + 1);
			self.buffer[(x + y * w) as usize] = col;
		}
	}
}

impl<T: GraphicsRenderer> GraphicsOperations for HeapBuffer<T> {}
