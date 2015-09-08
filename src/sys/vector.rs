use std::cmp::Eq;
use std::ops::{Add, Sub, Mul, Div};

pub trait VecBase: Eq + Add + Sub + Mul + Div + Sized {}

pub struct Vector<T: Eq + Add + Sub + Mul + Div> {
	x: T,
	y: T
}

impl<T: Eq + Add + Sub + Mul + Div> Vector<T> {
	/// # Create a new vector
	///
	/// This creates a new 2Dimensional vector with the specified starting points.
	pub fn new(x: T, y: T) -> Vector<T> {
		Vector {
			x: x,
			y: y
		}
	}

	// TODO: Implement all the beautiful traits and some other useful functions.
}
