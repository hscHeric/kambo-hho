use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Report {
    pub best_fitness: f64,
    pub best_position: Vec<f64>,
    pub convergence_curve: Vec<f64>,
    pub iters: usize,
    pub evals: usize,
    pub duration: Option<Duration>,
}

impl Report {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            best_fitness: f64::INFINITY,
            best_position: Vec::new(),
            convergence_curve: Vec::new(),
            iters: 0,
            evals: 0,
            duration: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn last(&self) -> Option<f64> {
        self.convergence_curve.last().copied()
    }

    #[inline]
    #[must_use]
    pub const fn with_duration(mut self, d: Duration) -> Self {
        self.duration = Some(d);
        self
    }
}
