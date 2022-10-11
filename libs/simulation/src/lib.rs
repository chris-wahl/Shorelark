use std::sync::mpsc::channel;
use nalgebra as na;
use rand::{Rng, RngCore};

pub use crate::{
    animal::Animal,
    food::Food,
    world::World,
};

mod animal;
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

    pub fn step(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(animal.speed, 0.0);
            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
}
