pub trait Objective {
    type Fitness: Copy + PartialOrd + Send + Sync;
    fn better(a: Self::Fitness, b: Self::Fitness) -> bool;
}

pub struct Minimization;
pub struct Maximization;

impl Objective for Minimization {
    type Fitness = f64;

    #[inline]
    fn better(a: Self::Fitness, b: Self::Fitness) -> bool {
        b > a
    }
}

impl Objective for Maximization {
    type Fitness = f64;

    fn better(a: Self::Fitness, b: Self::Fitness) -> bool {
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
