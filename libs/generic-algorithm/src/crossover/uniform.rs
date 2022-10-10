use rand::{Rng, RngCore};

use crate::{Chromosome, CrossoverMethod};

#[derive(Clone, Debug)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        return Self;
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(&self,
                 rng: &mut dyn RngCore,
                 parent_a: &Chromosome,
                 parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        return parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect();
    }
}