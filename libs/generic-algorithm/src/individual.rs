use crate::Chromosome;

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub enum TestIndividual {
    /// For tests that require access to chromosome
    WithChromosome { chromosome: Chromosome },
    /// For tests that don't required access to chromosome
    WithFitness { fitness: f32 },
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        return Self::WithFitness { fitness };
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn create(chromosome: Chromosome) -> Self {
        return Self::WithChromosome { chromosome };
    }

    fn chromosome(&self) -> &Chromosome {
        return match self {
            Self::WithChromosome { chromosome } => chromosome,
            Self::WithFitness { .. } => panic!("Not supported for TestIndividual::WithFitness"),
        };
    }

    fn fitness(&self) -> f32 {
        return match self {
            Self::WithChromosome { chromosome } => chromosome.iter().sum::<f32>(), // The fitness function here is simply just summing up all the genes
            Self::WithFitness { fitness } => *fitness,
        };
    }
}
