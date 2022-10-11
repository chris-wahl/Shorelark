use nalgebra as na;
use rand::{Rng, RngCore};

pub use crate::{
    animal::Animal,
    food::Food,
    world::World,
};

mod animal;
mod eye;
mod food;
mod world;

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        return Self { world: World::random(rng) };
    }

    pub fn world(&self) -> &World {
        return &self.world;
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movements();
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(animal.speed, 0.0);
            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
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
                    food.position = rng.gen();
                }
            }

        }
    }
}
