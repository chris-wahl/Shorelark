use crate::*;

#[derive(Debug)]
pub struct Brain {
    pub(crate) nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        return Self {
            nn: nn::Network::random(rng, &Self::topology(eye))
        };
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        return self.nn.weights().into_iter().collect();
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, eye: &Eye) -> Self {
        return Self {
            nn: nn::Network::from_weights(&Self::topology(eye), chromosome)
        };
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        return [
            // The Input Layer
            //
            // Because our eye returns Vec<f32>, and our neural
            // network works on Vec<f32>, we can pass-through
            // numbers from eye into the neural network directly.
            nn::LayerTopology {
                neurons: eye.cells(),
            },

            // The Hidden Layer
            //
            // There is no best answer as to "how many neurons
            // the hidden layer should contain" (or how many
            // hidden layers there should be, even - there could
            // be zero, one, two or more!).
            //
            // The rule of thumb is to start with a single hidden
            // layer that has somewhat more neurons that the input
            // layer, and see how well the network performs.
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },

            // The Output Layer
            //
            // Since the brain will control our bird's speed and
            // rotation, this gives us two numbers = two neurons.
            nn::LayerTopology { neurons: 2 },
        ];
    }
}