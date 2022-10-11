use std::f32::consts::FRAC_PI_2;

use nalgebra as na;
use rand::{Rng, RngCore};

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;

use self::{
    animal_individual::*,
    brain::*,
};
pub use self::{
    animal::Animal,
    eye::Eye,
    food::Food,
    world::World,
};
pub use ga::Statistics;

mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;

// Non-zero speed prevents bird from being stuck in one place
const SPEED_MIN: f32 = 0.001;
// Prevent the bird from approaching the speed of causality `c`
const SPEED_MAX: f32 = 0.005;
// Prevent infinite accelerations in speed
const SPEED_ACCEL: f32 = 0.2;
// And in rotation
const ROTATION_ACCEL: f32 = FRAC_PI_2;
// Minimum number of steps before evolving the algorithm
const GENERATION_LENGTH: usize = 2500;


pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(0.01, 0.3),
        );

        return Self { world, ga, age: 0 };
    }

    pub fn world(&self) -> &World {
        return &self.world;
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<ga::Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;
        if self.age > GENERATION_LENGTH {
            return Some(self.evolve(rng));
        }
        return None;
    }

    /// Fast-forwards until the end of the current generation.
    pub fn train(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        // To reduce complexity, assuming birds are circulate, and just checking if radii between
        // bird and food is <= the sum of their radii.
        // If a collision is found, move the food to a random location ("eaten" and "spawned" a new one)
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision = animal.eye.process_vision(
                animal.position,
                animal.rotation,
                &self.world.foods,
            );

            let response = animal.brain.nn.propagate(vision);
            let speed_acceleration = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation_acceleration = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = (animal.speed + speed_acceleration).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation_acceleration);
            // rotation is already wrapped from [0, 2π]
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(animal.speed, 0.0);
            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        self.age = 0;

        // 1. Prepare the current population of birds to go into the GA (must have `Individual` trait
        let current_population: Vec<_> = self.world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        // 2. Evolve the population
        let (evolved_population, stats) = self.ga.evolve(rng, &current_population);

        // 3. Return from the GA
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        // 4. Restart food (not required; just makes it easier to see when an evolution happens in the UI)
        self.world.foods.iter_mut().for_each(|food| food.position = rng.gen());

        return stats;
    }
}
