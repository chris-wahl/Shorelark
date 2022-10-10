use std::iter::once;

use rand::{Rng, RngCore};

use self::{layer::*, neuron::*};
pub use self::layer_topology::*;

mod layer;
mod layer_topology;
mod neuron;

#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>,
}


impl Network {
    pub fn random(
        rng: &mut dyn rand::RngCore,
        layers: &[LayerTopology],
    ) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        return Self { layers };
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        return self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs));
    }
}



