pub use self::{
    individual::*,
    selection::*,
};

use rand::RngCore;

mod individual;
mod selection;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

impl<S> GeneticAlgorithm<S>
    where S: SelectionMethod
{
    pub fn new(selection_method: S) -> Self {
        return Self { selection_method };
    }

    pub fn evolve<I>(
        &self,
        rng: &mut dyn RngCore,
        population: &[I],
    ) -> Vec<I>
        where
            I: Individual
    {
        assert!(!population.is_empty());
        return (0..population.len())
            .map(|_| {
                // Selection
                let (parent_a, parent_b) = (
                    self.selection_method.select(rng, population),
                    self.selection_method.select(rng, population)
                );


                // TODO: crossover
                // TODO: mutation
                todo!()
            })
            .collect();
    }
}