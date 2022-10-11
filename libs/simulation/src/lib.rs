use std::f32::consts::FRAC_PI_2;

use nalgebra as na;
use rand::{Rng, RngCore};

use lib_neural_network as nn;

pub use crate::{
    animal::Animal,
    eye::Eye,
    food::Food,
    world::World,
};

mod animal;
mod eye;
mod food;
mod world;

const SPEED_MIN: f32 = 0.001;
// Non-zero speed prevents bird from being stuck in one place
const SPEED_MAX: f32 = 0.005;
// Prevent the bird from approaching the speed of causality `c`
const SPEED_ACCEL: f32 = 0.2;
// Prevent infinite accelerations in speed
const ROTATION_ACCEL: f32 = FRAC_PI_2; // And in rotation


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
        self.process_brains();
        self.process_movements();
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

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision = animal.eye.process_vision(
                animal.position,
                animal.rotation,
                &self.world.foods,
            );

            let response = animal.brain.propagate(vision);
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
}
