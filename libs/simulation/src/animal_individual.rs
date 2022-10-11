use crate::*;

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        return Self {
            fitness: animal.satiation as f32,
            chromosome: animal.as_chromosome()
        }
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        return Animal::from_chromosome(self.chromosome, rng);
    }
}

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        return Self { fitness: 0.0, chromosome };
    }

    fn chromosome(&self) -> &ga::Chromosome {
        return &self.chromosome;
    }

    fn fitness(&self) -> f32 {
        return self.fitness;
    }
}