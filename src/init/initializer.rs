use rand::Rng;
use thiserror::Error;

use crate::core::Bounds;

#[derive(Debug, Error)]
pub enum InitError {
    #[error("invalid population size: {0}")]
    InvalidPopSize(usize),
}

pub trait Initializer: Send + Sync {
    /// # Errors
    ///
    /// Retorna um error se o tamanho da população for igual a 0
    fn initialize<R: Rng>(
        &self,
        pop_size: usize,
        bounds: &Bounds,
        rng: &mut R,
    ) -> Result<Vec<Vec<f64>>, InitError>;
}
