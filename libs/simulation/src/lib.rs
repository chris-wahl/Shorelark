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
}
