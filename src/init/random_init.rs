use rand::Rng;

use crate::{
    core::Bounds,
    init::{InitError, Initializer},
};

#[derive(Default, Clone, Copy, Debug)]
pub struct RandomInitializer;

impl Initializer for RandomInitializer {
    fn initialize<R: Rng>(
        &self,
        pop_size: usize,
        bounds: &Bounds,
        rng: &mut R,
    ) -> Result<Vec<Vec<f64>>, InitError> {
        if pop_size == 0 {
            return Err(InitError::InvalidPopSize(pop_size));
        }
        let dim = bounds.dim();
        let mut positions = Vec::with_capacity(pop_size);

        for _ in 0..pop_size {
            let mut x = bounds.gen_random_vec(rng);
            debug_assert_eq!(x.len(), dim);
            bounds.project_slice(&mut x);
            positions.push(x);
        }
        Ok(positions)
    }
}
