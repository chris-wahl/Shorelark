extern crate core;

use rand::Rng;

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

    pub fn weights(&self) -> Vec<f32> {
        use std::iter::once;
        // For each layer in layers,
        //      For each neuron in the layer
        //          - push the neuron's bias, and then each of its its weights
        // returning a Vector of f32s.
        // Just flattening the bias and all the weights of each neuron in each layer
        return self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .cloned()
            .collect();
    }

    pub fn from_weights(
        layers: &[LayerTopology],
        weights: impl IntoIterator<Item=f32>,
    ) -> Self {
        assert!(layers.len() > 1);
        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("Got too many weights");
        }
        return Self { layers };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod weights {
        use super::*;

        #[test]
        fn test() {
            let network = Network {
                layers: vec![
                    Layer { neurons: vec![Neuron { bias: 0.1, weights: vec![0.2, 0.3, 0.4] }] },
                    Layer { neurons: vec![Neuron { bias: 0.5, weights: vec![0.6, 0.7, 0.8] }] },
                ]
            };

            let actual = network.weights();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            approx::assert_relative_eq!(
                actual.as_slice(),
                expected.as_slice(),
            );
        }
    }

    mod from_weights {
        use super::*;

        #[test]
        fn test() {
            let layers = &[
                LayerTopology { neurons: 3 },
                LayerTopology { neurons: 2 },
            ];

            let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            let network = Network::from_weights(layers, weights.clone());
            let actual: Vec<_> = network.weights().into_iter().collect();

            approx::assert_relative_eq!(
                actual.as_slice(),
                weights.as_slice(),
            );
        }
    }
}

