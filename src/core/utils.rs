use std::cmp::Ordering;

use crate::core::{Decoder, Objective};

/// # Panics
///
/// Lança um Panic decoder failed caso aconteça algum erro durante a execução de docoder
#[inline]
pub fn evaluate_all(decoder: &dyn Decoder, positions: &[Vec<f64>]) -> Vec<f64> {
    positions
        .iter()
        .map(|x| decoder.decode(x).expect("decoder failed"))
        .collect()
}

#[inline]
#[must_use]
pub fn best_index_with<O>(fitness: &[f64]) -> usize
where
    O: Objective,
{
    let mut best_i = 0;
    let mut best_f = fitness[0];
    (1..fitness.len()).for_each(|i| {
        if O::better(fitness[i], best_f) {
            best_i = i;
            best_f = fitness[i];
        }
    });
    best_i
}

/// # Panics
/// Para a execução se o vetor se o tamanho do vetor de fitness for diferente do tamanho do vetor
/// de hawks
pub fn sort_by_fitness_with<O>(positions: &mut Vec<Vec<f64>>, fitness: &mut Vec<f64>)
where
    O: Objective,
{
    assert_eq!(positions.len(), fitness.len(), "len mismatch");
    let n = fitness.len();
    let mut idx: Vec<usize> = (0..n).collect();

    idx.sort_by(|&i, &j| cmp_fitness::<O>(fitness[i], fitness[j]));

    let mut new_pos = Vec::with_capacity(n);
    let mut new_fit = Vec::with_capacity(n);
    for &k in &idx {
        new_pos.push(positions[k].clone());
        new_fit.push(fitness[k]);
    }
    *positions = new_pos;
    *fitness = new_fit;
}

#[inline]
fn cmp_fitness<O>(a: f64, b: f64) -> Ordering
where
    O: Objective,
{
    if O::better(a, b) {
        Ordering::Less
    } else if O::better(b, a) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
