#![feature(type_alias_impl_trait)]

extern crate core;

use rand::RngCore;

pub use self::{
    chromosome::*,
    crossover::*,
    individual::*,
    mutation::*,
    selection::*,
};

mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
    where S: SelectionMethod
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static
    ) -> Self {
        return Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        };
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

                // Crossover genes
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                // Mutate genes
                self.mutation_method.mutate(rng, &mut child);

                return I::create(child);
            })
            .collect();
    }
}