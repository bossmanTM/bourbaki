use std::ops::{Add, Sub, Mul, Div}; 

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
		let mut diff:Cartesian<T> = Cartesian::new();
		for i in 0..self.coords.len() {
			diff.coords.push(self.coords[i] - rhs.coords[i])
		}
		diff
	}
}

// im an idiot and forgot that it doesnt work that way lol
////implimenting generic mul for Cartesian
//impl<T: Mul<Output = T> + Copy> Mul for Cartesian<T> {
//	type Output = Self;
//
//	fn mul(self, rhs: Self) -> Self::Output {
//		let mut prod:Cartesian<T> = Cartesian::new();
//		for i in 0..self.coords.len() {
//			prod.coords.push(self.coords[i] + rhs.coords[i])
//		}
//		prod
//	}
//}
//
////implimenting generic mul for Cartesian
//impl<T: Div<Output = T> + Copy> Div for Cartesian<T> {
//	type Output = Self;
//
//	fn div(self, rhs: Self) -> Self::Output {
//		let mut quot:Cartesian<T> = Cartesian::new();
//		for i in 0..self.coords.len() {
//			quot.coords.push(self.coords[i] + rhs.coords[i])
//		}
//		quot
//	}
//}