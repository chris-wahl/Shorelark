use rand::prelude::*;

pub struct Network {
    layers: Vec<Layer>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
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

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        return self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect();
    }

    pub fn random(
        rng: &mut dyn rand::RngCore,
        input_neurons: usize,
        output_neurons: usize,
    ) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(rng, input_neurons))
            .collect();

        return Self { neurons };
    }
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        return (self.bias + output).max(0.0);
    }

    pub fn random(
        rng: &mut dyn rand::RngCore,
        output_size: usize,
    ) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();
        return Self { bias, weights };
    }
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    mod random {
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use super::*;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);
            assert_relative_eq!(neuron.bias, -0.6255188);
            assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
            );
        }
    }

    mod propagate {
        use crate::Neuron;

        use super::*;

        #[test]
        fn test() {
            let bias = 0.5;
            let weights = vec![-0.3, 0.8];

            let neuron = Neuron { bias, weights: weights.clone() };

            // Ensures `.max()` limitation works
            assert_relative_eq!(
                neuron.propagate(&[-10.0, -10.0]),
                0.0
            );

            // Calculate explicitly with known values
            let inputs = vec![0.5, 1.0];
            assert_relative_eq!(
                neuron.propagate(&inputs),
                (weights[0] * inputs[0]) + (weights[1] * inputs[1]) + bias
            );
        }
    }
}