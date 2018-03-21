extern crate sdl2;

use utils::color::RGBColor;

/// A buffer for pixel drawing.
pub struct Canvas<'a> {
	buffer: &'a mut [u8],
	pitch: usize,
	height: usize,
	width: usize
}

impl<'a> Canvas<'a> {
	/// Creates a new drawing buffer for with the given height and width.
	pub fn new(buffer: &'a mut [u8], pitch: usize, width: usize, height: usize) -> Canvas<'a> {
		Canvas {buffer, pitch, width, height}
	}

	/// The width of the canvas
	pub fn width(&self) -> usize {
		self.width
	}

	/// The height of the canvas
	pub fn height(&self) -> usize {
		self.height
	}

	/// Draws a pixel at the given coordinates.
	pub fn draw_pixel(&mut self, x: usize, y: usize, color: &RGBColor) {
		let offset = y * self.pitch + x * 3;
		self.buffer[offset] = color.red;
		self.buffer[offset + 1] = color.green;
        self.buffer[offset + 2] = color.blue;
	}
}
