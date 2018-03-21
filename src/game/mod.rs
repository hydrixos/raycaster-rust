pub mod map;
pub mod player;

use game::map::Tile;
use game::map::TilePosition;
use game::map::Map;
use game::player::Player;

/// Represents the state of our game's virtual world
pub struct Game {
	/// The map of our virtual world
	pub map: Map,

	/// The player of our virtual world
	pub player: Player
}

impl Game {
	/// Initializes a new game based on a given map and player.
	pub fn new(map: Map, player: Player) -> Game {
		Game {map, player}
	}

	/// Rotates the player's viewing angle with the given angle.
	///
	/// # Parameters:
	///		- `angle`:		The angle the player should rotated with (0…2π).
	pub fn rotate_player(&mut self, step: f64) {
		self.player.direction += step;
	}		

	/// Moves the player by the given distance in its current viewing direction. The player is not moved if it would collide with a wall.
	///
	/// # Parameters:
	///		- `distance:		The distance the player should be moved by.
	pub fn move_player(&mut self, distance: f64) {
		// The actual distance is the hypothenuse of a right-angled triangle. The legs are the differences in the x and y direction. Using the ray's angle we can determine the length of the legs.
		// See: https://en.wikipedia.org/wiki/Trigonometry#Overview	
		let new_position = self.player.position.add(distance, self.player.direction);

		match self.map.tile(&TilePosition::new(&new_position, &self.player.direction)) {
			Tile::Empty => {self.player.position = new_position},
			Tile::Wall(_) => {}
		}
	}
}