use std::ops::{Add, Sub}; 

pub struct Cartesian<T> {
	pub coords :Vec<T>
}

impl<T> Cartesian<T> {
	pub fn new() -> Cartesian<T> {
		Cartesian {coords : Vec::new()}
	}
}

//implimenting generic add for Cartesian
impl<T: Add<Output = T> + Copy> Add for Cartesian<T> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut sum:Cartesian<T> = Cartesian::new();
		for i in 0..self.coords.len() {
			sum.coords.push(self.coords[i] + rhs.coords[i])
		}
		sum
	}
}

//implimenting generic sub for Cartesian
impl<T: Sub<Output = T> + Copy> Sub for Cartesian<T> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let mut sum:Cartesian<T> = Cartesian::new();
		for i in 0..self.coords.len() {
			sum.coords.push(self.coords[i] - rhs.coords[i])
		}
		sum
	}
}