use utils::geometry::Angle;
use utils::geometry::Point;

/// Represents a player inside the map.
pub struct Player {
	/// The player's position inside the map.
	pub position:		Point,

	/// The player's viewing angle (relative to the x-axis).
	pub direction:		Angle
}
