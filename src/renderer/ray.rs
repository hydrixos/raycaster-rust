use utils::geometry::Angle;
use utils::geometry::Axis;
use utils::geometry::Direction;
use utils::geometry::Point;

/// Describes a Ray that move through a map
pub struct Ray {
	/// The starting point of the ray
	pub start:	Point,

	/// The ending point of the ray
	pub end:	Point,
	
	/// The angle of the ray
	pub angle:	Angle,

	/// The length of the ray
	pub length:	f64
}

// Methods related to ray casting
impl Ray {
	/// Initializes a ray with a starting point and an angle. The ray's end will be set to its start.
	pub fn new(start: Point, angle: f64) -> Ray {
		Ray::new_with_end(start.clone(), start, angle)
	}

	/// Initializes a ray with a start position, an end position and an angle.
	fn new_with_end(start: Point, end: Point, angle: f64) -> Ray {
		let delta_x = end.x - start.x;
		let delta_y = end.y - start.y;
		let length = (delta_x * delta_x + delta_y * delta_y).sqrt();

		Ray {start, end, angle, length}
	}

	/// Creates a new ray where the end point moved one step to the next grid line that is parallel either to the X axis or Y axis.
	pub fn grow(&self) -> Ray {
		// Simulate the ray is hitting the next grid line that is parallel either to the X or to the Y axis
		let ray_on_next_x_line = self.grow_to_next_x_line();
		let ray_on_next_y_line = self.grow_to_next_y_line();

		// Choose the candidate with the shorter distance to the current point.
		if ray_on_next_x_line.length < ray_on_next_y_line.length {
			return ray_on_next_x_line
		}
		else {
			return ray_on_next_y_line
		}
	}

	/// Creates a new ray where the end point progressed to the next grid line that is parallel to the X axis.
	fn grow_to_next_x_line(&self) -> Ray {
		let delta_x = self.distance_to_next_grid_line(Axis::X);
		let delta_y = self.angle.tan() * delta_x;
		return self.grow_with_delta(delta_x, delta_y);
	}

	/// Creates a new ray where the end point progressed to the next grid line that is parallel to the Y axis.
	fn grow_to_next_y_line(&self) -> Ray {
		let delta_y = self.distance_to_next_grid_line(Axis::Y);
		let delta_x = delta_y / self.angle.tan();
		return self.grow_with_delta(delta_x, delta_y);
	}

	/// Moves the end point of the ray by the given delta.
	fn grow_with_delta(&self, delta_x: f64, delta_y: f64) -> Ray {
		Ray::new_with_end(self.start.clone(), Point {x: self.end.x + delta_x, y: self.end.y + delta_y}, self.angle)
	}

	/// Determines the shorted distance between the ray's endpoint and the next grid line that is parallel to the given axis and in the ray's direction.
	///
	/// # Parameters:
	///		- axis:		The axis the matching grid line should be parallel to
	fn distance_to_next_grid_line(&self, axis: Axis) -> f64 {
		let position = self.end.component(&axis);
		match Direction::from_angle(&self.angle, &axis) {
			Direction::Increasing => (position).floor() + 1.0 - position,
			Direction::Decreasing => position.ceil() - 1.0 - position
		}
	}
}
