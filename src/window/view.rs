extern crate sdl2;

use window::canvas::Canvas;

/// A screen tile that can be used for drawing. (E.g. the content tile of a window or a HTML canvas.)
pub struct View {
	canvas: sdl2::render::Canvas<sdl2::video::Window>
}

impl View {
	/// Creates a new view using an SDL context.
	pub fn new(sdl_context: &sdl2::Sdl) -> View {
    	let video_subsystem = sdl_context.video().unwrap();
    	let window = video_subsystem.window("Ray Casting Demo", 800, 600)
			.resizable().position_centered().allow_highdpi().opengl().build().unwrap();
    	let canvas = window.into_canvas()
        	.accelerated().present_vsync().build().unwrap();			
		
		View {canvas}
	}

	/// Creates a new canvas for draing a single frame and passes it to the given block. After the block has been finished, the canvas is drawn to the
	pub fn draw_canvas<F>(&mut self, drawing_function: F) where F: FnOnce(&mut Canvas), {
		let texture_creator = self.canvas.texture_creator();
		let width = self.canvas.window().size().0;
		let height = self.canvas.window().size().1;

		let mut texture = texture_creator
			.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, width, height)
			.unwrap();

		texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
			drawing_function(&mut Canvas::new(buffer, pitch, width as usize, height as usize));
		}).unwrap();

		self.canvas.copy(&texture, None, None).unwrap();
		self.canvas.present();
}
}
