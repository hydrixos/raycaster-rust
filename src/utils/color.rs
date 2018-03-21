/// Represents a RGB color
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RGBColor {
	/// The red component of a color
	pub red: u8,
	
	/// The green component of a color
	pub green: u8,

	/// The blue component of a color
	pub blue: u8
}

impl RGBColor {
	pub fn black() -> RGBColor { RGBColor {red: 0, green: 0, blue: 0} }
	pub fn dark_gray() -> RGBColor { RGBColor {red: 50, green: 50, blue: 50} }
	pub fn red() -> RGBColor { RGBColor {red: 180, green: 0, blue: 0} }
	pub fn green() -> RGBColor { RGBColor {red: 50, green: 128, blue: 0} }
	pub fn blue() -> RGBColor { RGBColor {red: 0, green: 64, blue: 128} }
	pub fn yellow() -> RGBColor { RGBColor {red: 255, green: 184, blue: 0} }
	pub fn orange() -> RGBColor { RGBColor {red: 255, green: 80, blue: 0} }
}

impl RGBColor {
	/// Creates a new color by darkening the color to the given percentage.
	pub fn adjust_light_intensity(&self, percentage: f64) -> RGBColor {
		RGBColor {
			red: 	RGBColor::darken_component(self.red, percentage),
			green:	RGBColor::darken_component(self.green, percentage),
			blue:	RGBColor::darken_component(self.blue, percentage)
		}
	}

	/// Determines the value of a certain color component when darkening a color.
	fn darken_component(component: u8, percentage: f64) -> u8 {
		(((component as f64) * percentage).max(0.0) as u8).min(component)
	}

}
