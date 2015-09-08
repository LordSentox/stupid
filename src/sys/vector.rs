use std::ops::{Add, Sub, Mul, Div};
use std::clone::Clone;

#[derive(Clone, Copy)]
pub struct Vector<T: PartialEq + Add + Sub + Mul + Div> {
	pub x: T,
	pub y: T
}

impl<T: PartialEq + Add<Output=T> + Sub + Mul<Output=T> + Div> Vector<T> {
	/// # Create a new vector
	///
	/// This creates a new 2Dimensional vector with the specified starting points.
	pub fn new(x: T, y: T) -> Vector<T> {
		Vector {
			x: x,
			y: y
		}
	}
}

/// Vector normalisation.
impl Vector<f32> {
	pub fn normalise(&self) -> Vector<f32> {
		self.clone() / (self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

/// Vector normalisation.
impl Vector<f64> {
	pub fn normalise(&self) -> Vector<f64> {
		self.clone() / (self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

impl<T: PartialEq + Add + Sub + Mul + Div> PartialEq for Vector<T> {
	fn eq(&self, other: &Vector<T>) -> bool {
		(self.x == other.x) &
		(self.y == other.y)
	}

	fn ne(&self, other: &Vector<T>) -> bool {
		(self.x != other.x) |
		(self.y != other.y)
	}
}

impl<T: PartialEq + Add<Output=T> + Sub + Mul + Div> Add for Vector<T> {
	type Output = Vector<T>;

	fn add(self, other: Vector<T>) -> Vector<T> {
		Vector {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}

impl<T: PartialEq + Add + Sub<Output=T> + Mul + Div> Sub for Vector<T> {
	type Output = Vector<T>;

	fn sub(self, other: Vector<T>) -> Vector<T> {
		Vector {
			x: self.x - other.x,
			y: self.y - other.y
		}
	}
}

impl<T: PartialEq + Add<Output=T> + Sub + Mul<Output=T> + Div> Mul for Vector<T> {
	type Output = T;

	fn mul(self, other: Vector<T>) -> T {
		self.x * other.x +
		self.y * other.y
	}
}

impl<T: PartialEq + Add + Sub + Mul<Output=T> + Div + Clone> Mul<T> for Vector<T> {
	type Output = Vector<T>;

	fn mul(self, other: T) -> Vector<T> {
		Vector {
			x: self.x * other.clone(),
			y: self.y * other
		}
	}
}

impl<T: PartialEq + Add + Sub + Mul + Div<Output=T> + Clone> Div<T> for Vector<T> {
	type Output = Vector<T>;

	fn div(self, other: T) -> Vector<T> {
		Vector {
			x: self.x / other.clone(),
			y: self.y / other
		}
	}
}
