/// Specifies an angle relative to the X-Axis in radians
pub type Angle = f64;

/// Specifies an axis in the coordinate system.
pub enum Axis {
	X,
	Y
}

/// Specifies a point within the map.
#[derive(Clone)]
pub struct Point {
	pub x: f64,
	pub y: f64
}

impl Point {
	/// Adds two points
	pub fn add(&self, distance: f64, direction: f64) -> Point {
		Point {x: self.x + distance * direction.cos(), y: self.y + distance * direction.sin()}
	}

	/// Provides the component of a point for an axis.
	pub fn component(&self, axis: &Axis) -> f64 {
		match *axis {
			Axis::X => self.x,
			Axis::Y => self.y
		}
	}

	/// Determines the axis the closets grid line is parallel to.
	pub fn closest_grid_line_axis(&self) -> Axis {
		if (self.x - self.x.round()).abs() < (self.y - self.y.round()).abs() {
			Axis::X
		}
		else {
			Axis::Y
		}
	}
}

/// Specifies the direction of a line on a certain axis.
pub enum Direction {
	Increasing,
	Decreasing
}

impl Direction {
	/// Determines whether a line with a certain angle will result in decreasing or increasing values on a given axis.
	pub fn from_angle(angle: &Angle, axis: &Axis) -> Direction {
		// Take a look at the unit circle: https://en.wikipedia.org/wiki/Unit_circle
		match *axis {
			// A line's values are increasing on the x-axis if the angle is between 270° and 90° (when cos(angle) > 0).
			Axis::X => if angle.cos() > 0.0 { Direction::Increasing } else { Direction::Decreasing },

			// A line's values are increasing on the y-axis if the angle is between 0° and 180° (when sin(angle) > 0).
			Axis::Y => if angle.sin() > 0.0 { Direction::Increasing } else { Direction::Decreasing }
		}
	}
}
