use rand::Rng;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BoundsError {
    #[error("Dimension mismatch: lo({lo}) != hi({hi})")]
    DimMismatch { lo: usize, hi: usize },

    #[error("Invalid interval at i={i}: lo={lo} > hi={hi}")]
    InvalidInterval { i: usize, lo: f64, hi: f64 },

    #[error("Invalid dimension: dim={dim}")]
    InvalidDim { dim: usize },
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Policy {
    #[default]
    Clamp,
}

#[derive(Clone, Debug)]
enum Spec {
    /// Limites fixos para todas as variaveis, ou seja, para cada falcão cada "gene" vai estar
    /// entre lo e hi
    Uniform { lo: f64, hi: f64, dim: usize },

    /// Cada gene tem um upper-bound e um lower-bound especifico
    PerDim { lo: Vec<f64>, hi: Vec<f64> },
}

#[derive(Clone, Debug)]
pub struct Bounds {
    spec: Spec,
    policy: Policy,
}

impl Bounds {
    /// Creates a **uniform** bound for all dimensions.
    ///
    /// Each dimension receives the same lower (`lo`) and upper (`hi`) limits.
    /// The number of dimensions is defined by `dim`.
    ///
    /// By default, the created bound uses the [`BoundaryPolicy::Clamp`] policy,
    /// meaning values outside the range are clamped to the nearest boundary.
    ///
    /// # Parameters
    ///
    /// - `lo`: lower bound.
    /// - `hi`: upper bound.
    /// - `dim`: number of dimensions (> 0).
    ///
    /// # Returns
    ///
    /// A [`Bounds`] instance with the uniform specification.
    ///
    /// # Errors
    ///
    /// - [`BoundsError::InvalidDim`] if `dim == 0`.
    /// - [`BoundsError::InvalidInterval`] if `lo > hi`.
    ///
    pub fn uniform(lo: f64, hi: f64, dim: usize) -> Result<Self, BoundsError> {
        if dim == 0 {
            return Err(BoundsError::InvalidDim { dim });
        }
        if lo > hi {
            return Err(BoundsError::InvalidInterval { i: 0, lo, hi });
        }
        Ok(Self {
            spec: Spec::Uniform { lo, hi, dim },
            policy: Policy::Clamp,
        })
    }

    /// Creates bounds with per-dimension lower and upper limits.
    ///
    /// Unlike [`Bounds::uniform`], this method allows each dimension to have its own
    /// lower (`lo[i]`) and upper (`hi[i]`) bounds.
    ///
    /// By default, the created bound uses the [`BoundaryPolicy::Clamp`] policy,
    /// meaning values outside the range are clamped to the nearest boundary.
    ///
    /// # Parameters
    ///
    /// - `lo`: vector of lower bounds, one per dimension.
    /// - `hi`: vector of upper bounds, one per dimension.
    ///
    /// # Returns
    ///
    /// A [`Bounds`] instance with per-dimension specification.
    ///
    /// # Errors
    ///
    /// - [`BoundsError::DimMismatch`] if `lo.len() != hi.len()`.
    /// - [`BoundsError::InvalidDim`] if `lo` and `hi` are empty.
    /// - [`BoundsError::InvalidInterval`] if for any index `i`, `lo[i] > hi[i]`.
    ///
    pub fn per_dim(lo: Vec<f64>, hi: Vec<f64>) -> Result<Self, BoundsError> {
        if lo.len() != hi.len() {
            return Err(BoundsError::DimMismatch {
                lo: lo.len(),
                hi: hi.len(),
            });
        }
        if lo.is_empty() {
            return Err(BoundsError::InvalidDim { dim: 0 });
        }
        for (i, (&l, &h)) in lo.iter().zip(&hi).enumerate() {
            if l > h {
                return Err(BoundsError::InvalidInterval { i, lo: l, hi: h });
            }
        }
        Ok(Self {
            spec: Spec::PerDim { lo, hi },
            policy: Policy::Clamp,
        })
    }

    /// Altera a politica de mapeamento das soluções
    #[must_use]
    pub const fn with_policy(mut self, policy: Policy) -> Self {
        self.policy = policy;
        self
    }

    #[inline]
    #[must_use]
    pub const fn dim(&self) -> usize {
        match &self.spec {
            Spec::Uniform { lo: _, hi: _, dim } => *dim,
            Spec::PerDim { lo, hi: _ } => lo.len(),
        }
    }

    #[inline]
    #[must_use]
    pub fn lo_at(&self, i: usize) -> f64 {
        match &self.spec {
            Spec::Uniform { lo, .. } => *lo,
            Spec::PerDim { lo, .. } => lo[i],
        }
    }

    #[inline]
    #[must_use]
    pub fn hi_at(&self, i: usize) -> f64 {
        match &self.spec {
            Spec::Uniform { hi, .. } => *hi,
            Spec::PerDim { hi, .. } => hi[i],
        }
    }

    #[inline]
    #[must_use]
    pub fn span_at(&self, i: usize) -> f64 {
        self.hi_at(i) - self.lo_at(i)
    }

    pub fn project_slice(&self, x: &mut [f64]) {
        debug_assert_eq!(x.len(), self.dim(), "slice dim != bounds dim");
        match (self.policy, &self.spec) {
            (Policy::Clamp, Spec::Uniform { lo, hi, dim: _ }) => {
                let (lo, hi) = (*lo, *hi);
                for xi in x {
                    *xi = xi.clamp(lo, hi);
                }
            }
            (Policy::Clamp, Spec::PerDim { lo, hi }) => {
                // Estou usando unsafe aqui apenas para que o rust pule as verificações, já que o
                // debug_assert_eq já faz a verificação de que as dimensões são iguais
                for i in 0..x.len() {
                    let l = unsafe { *lo.get_unchecked(i) };
                    let h = unsafe { *hi.get_unchecked(i) };
                    // clamp
                    let xi = unsafe { x.get_unchecked_mut(i) };
                    *xi = (*xi).clamp(l, h);
                }
            }
        }
    }

    pub fn gen_random_vec<R: Rng>(&self, rng: &mut R) -> Vec<f64> {
        let d = self.dim();
        let mut s = Vec::with_capacity(d);

        match &self.spec {
            Spec::Uniform { lo, hi, dim } => {
                for _ in 0..*dim {
                    let random_val = rng.random_range(*lo..=*hi);
                    s.push(random_val);
                }
            }
            Spec::PerDim { lo, hi } => {
                for i in 0..d {
                    let random_val = rng.random_range(lo[i]..=hi[i]);
                    s.push(random_val);
                }
            }
        }

        s
    }
}
