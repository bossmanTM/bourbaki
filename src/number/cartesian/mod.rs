use num::Signed;


pub struct Cartesian<T> {
	pub coords :Vec<T>
}

impl<T> Cartesian<T> {
	pub fn new() -> Cartesian<T> {
		Cartesian {coords : Vec::new()}
	}
}

//impl<T: Signed> Signed for Cartesian<T> {
//	fn abs(&self) -> Cartesian<T> 
//	{
//		let mut abs_num = Cartesian::new();
//		for i in &self.coords {
//			abs_num.coords.push(i.abs())
//		}
//		abs_num
//	}
//
//	fn abs_sub(&self, other: &Self) -> Self {
//		let mut abs_num = Cartesian::new();
//		for i in 0..self.coords.len() {
//			abs_num.coords.push(self.coords[i].abs_sub(&other.coords[i]));
//		}
//		abs_num
//	}
//
//	fn signum(&self) -> Self {
//		let mut signum_num = Cartesian::new();
//		for i in &self.coords {
//			signum_num.coords.push(i.signum());
//		}
//		signum_num
//	}
//
//	fn is_negative(&self) -> bool {
//		let mut negative = false;
//		for i in &self.coords {
//			negative = i.is_negative() || negative
//		}
//		negative
//	}
//
//	fn is_positive(&self) -> bool {
//		let mut negative = false;
//		for i in &self.coords {
//			negative = i.is_negative() || negative
//		}
//		negative
//	}
//}