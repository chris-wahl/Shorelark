use crate::{na, Rng, RngCore};


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
