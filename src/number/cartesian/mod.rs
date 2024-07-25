use std::ops::{Add, Sub}; 

pub struct Cartesian<T> {
	pub coords :Vec<T>
}

impl<T> Cartesian<T> {
	pub fn new() -> Cartesian<T> {
		Cartesian {coords : Vec::new()}
	}
}


//implimenting add for my cartesian coordinate system
macro_rules! impl_add {
	($($t:ty)*) => ($(
		impl Add for Cartesian< $t > {
			type Output = Cartesian<$t>;
			
			fn add(self, rhs: Self) -> Self::Output {
				let mut sum:Cartesian<$t> = Cartesian::new();
				for i in 0..self.coords.len() {
					sum.coords.push(&self.coords[i] + &rhs.coords[i])
				}
				sum
			}
		}
		
	)*)
}

// apparently i need to impliment it one at a time, ig rust aint got it all
impl_add! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

//implimenting subtraction for my cartesian coordinate system
macro_rules! impl_sub {
	($($t:ty)*) => ($(
		impl Sub for Cartesian< $t > {
			type Output = Cartesian<$t>;
			
			fn sub(self, rhs: Self) -> Self::Output {
				let mut sum:Cartesian<$t> = Cartesian::new();
				for i in 0..self.coords.len() {
					sum.coords.push(&self.coords[i] - &rhs.coords[i])
				}
				sum
			}
		}
		
	)*)
}

// apparently i need to impliment it one at a time, ig rust aint got it all
impl_sub! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }