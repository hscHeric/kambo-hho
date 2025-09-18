use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecoderError {
    #[error("Invalid dimension: expected size was {expected}, but received {received}")]
    InvalidDimension { expected: usize, received: usize },

    #[error("Value out of bounds: the value {value} is outside the range [0, {upper_bound}]")]
    OutOfBounds { value: usize, upper_bound: usize },

    #[error("Unknown error while decoding the solution")]
    UnknownError,
}

pub trait Decoder {
    /// Decodes the given `solution` vector into a valid solution and returns its fitness.
    ///
    /// The `solution` vector is expected to contain `f64` values that need to be
    /// mapped and validated. The implementation of this method should:
    ///
    /// 1. Validate the dimensions of the input `solution` vector.
    /// 2. Ensure that the values within the vector are within the established limits
    /// 3. Transform the raw `f64` values into a valid problem-specific solution.
    /// 4. Calculate the fitness of the valid solution.
    ///
    /// # Parameters
    ///
    /// - `solution`: A slice of `f64` values representing the encoded solution.
    ///
    /// # Errors
    ///
    /// This method will return a `DecoderError` if:
    /// - The `solution` vector has an invalid dimension.
    /// - Any value within the `solution` vector is out of the established bounds.
    /// - An unknown error occurs during decoding.
    fn decode(&self, solution: &[f64]) -> Result<f64, DecoderError>;
}
