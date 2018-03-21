extern crate sdl2;

pub mod event;
pub mod view;
pub mod canvas;

use window::view::View;
use window::event::EventSource;

/// A window that can be used for drawing and that provides user events. May be a window or a HTML canvas.
pub struct Window {
	/// The view that can be used for drawing
	pub view: View,

	/// An event source
	pub event_source: EventSource
}

impl Window {
	/// Creates a new window.
	pub fn new() -> Window {
		let sdl_context = sdl2::init().unwrap();
	    
	    let view = View::new(&sdl_context);
	    let event_source = EventSource::new(&sdl_context);

		Window {view, event_source}
	}
}
