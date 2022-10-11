use crate::{na, Rng, RngCore};

#[derive(Debug)]
pub struct Food {
    pub(crate) position: na::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        return Self { position: rng.gen() };
    }

    pub fn position(&self) -> na::Point2<f32> {
        return self.position;
    }
}
