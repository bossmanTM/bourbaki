pub use number as Number;

pub mod number;

pub struct Test {}

pub fn add(left: f64, right: f64) -> f64 {
    let a : number::Polar<f64> = number::Polar {radius : left, theta : vec![1.0]};
    let b : number::Polar<f64> = number::Polar {radius : right, theta : vec![1.0]};
    a.radius + b.radius
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
}
