use nalgebra as na;
use rand::{Rng, RngCore};

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

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40)
            .map(|_| Animal::random(rng))
            .collect();

        let foods = (0..60)
            .map(|_| Food::random(rng))
            .collect();

        return Self { animals, foods };
    }
    pub fn animals(&self) -> &[Animal] {
        return &self.animals;
    }
    pub fn foods(&self) -> &[Food] {
        return &self.foods;
    }
}


#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        return Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
        };
    }

    pub fn position(&self) -> na::Point2<f32> {
        return self.position;
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        return self.rotation;
    }
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        return Self { position: rng.gen() };
    }

    pub fn position(&self) -> na::Point2<f32> {
        return self.position;
    }
}
