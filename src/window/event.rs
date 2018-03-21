extern crate sdl2;

use sdl2::event::Event as SDLEvent;
use sdl2::event::WindowEvent as SDLWindowEvent;
use sdl2::keyboard::Keycode as SDLKeycode;

/// Wrapper around SDL to provide events
pub struct EventSource {
	event_pump: sdl2::EventPump
}

impl EventSource {
	/// Creates a new event source from a given SDL context.
	pub fn new(sdl_context: &sdl2::Sdl) -> EventSource {
		let event_pump = sdl_context.event_pump().unwrap();
		EventSource {event_pump: event_pump}
	}

	/// Polls the next event from the source.
	pub fn poll_next_event(&mut self) -> Option<Event> {
		if let Some(event) = self.event_pump.poll_iter().next() {
			match event {
				SDLEvent::Quit {..} | SDLEvent::KeyDown {keycode: Some(SDLKeycode::Escape), ..} => {
					return Some(Event::Quit)
				},
				SDLEvent::Window {win_event, ..} => {
					match win_event {
						SDLWindowEvent::Resized(..) | SDLWindowEvent::SizeChanged(..) => return Some(Event::Resize),
						_ => None
					}					
				}
				_ => None
			}
		}
		else {
			None
		}
	}

	/// Provides a set of key codes for the currently pressed keys.
	pub fn pressed_keycodes(&self) -> Vec<Keycode> {
		self.event_pump
			.keyboard_state()
			.pressed_scancodes()
			.filter_map({|scancode| 
				if let Some(sdl_keycode) = SDLKeycode::from_scancode(scancode) {
					Keycode::from_sdl_keycode(sdl_keycode)
				}
				else {
					None
				}

			})
			.collect()
	}
}

/// Possible events emitted by the source.
pub enum Event {
	/// The application should be terminated.
	Quit,

	/// The window was resized.
	Resize
}

/// Keycodes to detect pressed keys
#[derive(Debug)]
pub enum Keycode {
	Left, Right, Up, Down
}

impl Keycode {
	/// Converts an SDL Keycode to an abstraction layer keycode.
	fn from_sdl_keycode(sdl_keycode: SDLKeycode) -> Option<Keycode> {
		match sdl_keycode {
			SDLKeycode::Left => Some(Keycode::Left),
			SDLKeycode::Right => Some(Keycode::Right),
			SDLKeycode::Up => Some(Keycode::Up),
			SDLKeycode::Down => Some(Keycode::Down),
			_ => None
		}
	}
}
