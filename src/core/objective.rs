pub trait Objective {
    fn better(a: f64, b: f64) -> bool;
}

pub struct Minimization;
pub struct Maximization;

impl Objective for Minimization {
    #[inline]
    fn better(a: f64, b: f64) -> bool {
        a < b
    }
}

impl Objective for Maximization {
    fn better(a: f64, b: f64) -> bool {
        a > b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximization_minimization_test() {
        let a: f64 = 1000.0;
        let b: f64 = 999.0;

        assert!(Maximization::better(a, b));
        assert!(!Maximization::better(b, a));

        assert!(Minimization::better(b, a));
        assert!(!Minimization::better(a, b));
    }
}
