#![feature(type_alias_impl_trait)]

use rand::RngCore;

pub use self::{
    chromosome::*,
    crossover::*,
    individual::*,
    selection::*,
};

mod chromosome;
mod crossover;
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
                let [parent_a, parent_b] = [0, 1].map(|_| self.selection_method.select(rng, population).chromosome());


                // TODO: crossover
                // TODO: mutation
                // TODO: Convert `Chromosome` back to `Individual`
                todo!()
            })
            .collect();
    }
}