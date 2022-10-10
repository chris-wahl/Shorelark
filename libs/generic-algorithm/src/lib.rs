pub use self::{
    individual::*,
    selection::*,
};

mod individual;
mod selection;

pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn new() -> Self { return Self; }

    pub fn evolve<I, S>(
        &self,
        population: &[I],
        selection_method: &S,
    ) -> Vec<I>
        where
            I: Individual,
            S: SelectionMethod
    {
        assert!(!population.is_empty());
        return (0..population.len())
            .map(|_| {
                // TODO: selection
                // TODO: crossover
                // TODO: mutation
                todo!()
            })
            .collect();
    }
}