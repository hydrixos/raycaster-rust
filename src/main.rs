extern crate sdl2;

mod emscripten;
mod game;
mod renderer;
mod utils;
mod window;

use game::Game;
use game::map::Map;
use game::player::Player;
use renderer::Renderer;
use std::process::exit;
use std::time::Duration;
use utils::geometry::Point;
use window::Window;
use window::event::Event;
use window::event::Keycode;

fn main() {
	// Initialize the graphics and event handling.
	let mut window = Window::new();

	// Load the game and place the player within the map.
	let game = Game::new(
		Map::new(include_str!("../assets/map.txt")),
		Player {position: Point {x: 4.5, y: 5.5}, direction: 0.0}
	);

	// Initialize the renderer
	let mut renderer = Renderer::new(game, 1.0, 0.75, 100.0, 0.25);

	// Require a screen refresh after startup.	
	let mut initial_run = true;

	// Run main loop on normal UI targets.
    #[cfg(not(target_os = "emscripten"))]
    loop { 
    	main_loop(&mut window, &mut renderer, &initial_run);
    	initial_run = false;
    }

    // Run main loop on web targets.
    #[cfg(target_os = "emscripten")] {
	    use emscripten::{emscripten};
	    emscripten::set_main_loop_callback(|| {
	    	main_loop(&mut window, &mut renderer, &initial_run);
	    	initial_run = false;
	    });
	}    
}

/// The main event handling loop.
fn main_loop(window: &mut Window, renderer: &mut Renderer, initial_run: &bool) {
	let mut needs_refresh = *initial_run;

	// Get pending UI events
	match window.event_source.poll_next_event() {
		None => {},
		Some(Event::Quit) => { exit(0); },
		Some(Event::Resize) => { needs_refresh = true; }
	}

	// Handle key presses
	for keycode in window.event_source.pressed_keycodes() {
		needs_refresh = true;
		let movement_speed = 0.2;
		let rotation_speed = 0.05;

		match keycode {
			// Arrow up/down: Move player forward/backwards
			Keycode::Up => { renderer.game.move_player(movement_speed) },
			Keycode::Down => { renderer.game.move_player(-movement_speed) },
				
			// Arrow right/left: Rotate player
			Keycode::Right => { renderer.game.rotate_player(rotation_speed); },
			Keycode::Left => { renderer.game.rotate_player(-rotation_speed); },
		}
	}

	// Refresh screen if needed 
	if needs_refresh {
		window.view.draw_canvas({|mut canvas| 
			renderer.render(&mut canvas)
		});
	}

	std::thread::sleep(Duration::from_millis(1000/60));
}
