pub use gaussian::GaussianMutation;
use rand::RngCore;

use crate::Chromosome;

mod gaussian;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}