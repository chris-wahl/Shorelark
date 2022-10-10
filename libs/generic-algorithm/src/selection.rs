use rand::RngCore;

use crate::individual::Individual;

mod roulette_wheel;

pub trait SelectionMethod {
    fn select<'a, I>(
        &self,
        rng: &mut dyn RngCore,
        population: &'a [I],
    ) -> &'a I
        where
            I: Individual;
}