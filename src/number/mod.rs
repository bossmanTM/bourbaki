mod polar;
mod cartesian;

pub use polar::Polar as Polar;
pub use cartesian::Cartesian as Cartesian;

pub enum Number<T> {
	Polar(Polar<T>),
	Cartesian(Cartesian<T>)
}

pub struct Set<T> {
	pub condition :fn(Number<T>) -> bool
}

pub enum Operation {

}