use crate::*;


#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,

    pub (crate) eye: Eye,
    pub (crate) brain: nn::Network,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = nn::Network::random(
            rng,
            &[
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
            ]
        );

        return Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
        };
    }

    pub fn position(&self) -> na::Point2<f32> {
        return self.position;
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        return self.rotation;
    }
}
