#![feature(type_alias_impl_trait)]

extern crate core;

use rand::RngCore;

pub use self::{
    chromosome::Chromosome,
    crossover::UniformCrossover,
    individual::Individual,
    mutation::GaussianMutation,
    selection::RouletteWheelSelection,
    statistics::Statistics,
};

use self::{
    crossover::*,
    mutation::*,
    selection::*,
};
#[cfg(test)]
use self::individual::TestIndividual;

mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;
mod statistics;

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
        mutation_method: impl MutationMethod + 'static,
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
    ) -> (Vec<I>, Statistics)
        where
            I: Individual
    {
        assert!(!population.is_empty());
        let new_population = (0..population.len())
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
        let stats = Statistics::new(population);
        return (new_population, stats);
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();
        return TestIndividual::create(chromosome);
    }

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
        ];

        // Running `.evolve()` a few times to the differences between initial and output populations
        // are easier to spot.
        // Running 10 generations for no other particular reason.
        for _ in 0..10 {
            let (new_pop, _) = ga.evolve(&mut rng, &population);
            population = new_pop;
        }

        // Expect all fitness-es to have increased
        let expected_population = vec![
            individual(&[0.44769490, 2.0648358, 4.3058133]), // fitness ~= 6.8
            individual(&[1.21268670, 1.5538777, 2.8869110]), // fitness ~= 5.7
            individual(&[1.06176780, 2.2657390, 4.4287640]), // fitness ~= 7.8
            individual(&[0.95909685, 2.4618788, 4.0247330]), // fitness ~= 7.4
        ];

        assert_eq!(population, expected_population);
    }
}