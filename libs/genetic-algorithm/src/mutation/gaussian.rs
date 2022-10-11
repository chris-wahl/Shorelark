use rand::{Rng, RngCore};

use crate::{Chromosome, MutationMethod};

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    /// Probability of changing a gene:
    /// - 0.0 = no genes will be touched
    /// - 1.0 = all genes will be touched
    chance: f32,

    /// Magnitude of that change:
    /// - 0.0 = touched genes will not be modified
    /// - 3.0 = touched genes will be += or -= by at most 3.0
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);
        return Self { chance, coeff };
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn original_genes() -> Vec<f32> {
        return vec![1.0, 2.0, 3.0, 4.0, 5.0];
    }

    fn actual(chance: f32, coeff: f32) -> Vec<f32> {
        let mut child = original_genes().into_iter().collect();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);
        return child.into_iter().collect();
    }

    mod given_zero_chance {
        use super::original_genes;
        use approx;
        fn actual(coeff: f32) -> Vec<f32> {
            return super::actual(0.0, coeff);
        }

        mod and_zero_coefficient {
            use super::*;
            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = original_genes();

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_non_zero_coefficient {
            use super::*;
            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = original_genes();
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_fifty_fifty_chance {
        use super::original_genes;
        use approx;

        fn actual(coeff: f32) -> Vec<f32> {
            return super::actual(0.5, coeff);
        }

        mod and_zero_coefficient {
            use super::*;
            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = original_genes();
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_non_zero_coefficient {
            use super::*;
            #[test]
            fn slightly_changes_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_max_chance {
        use super::original_genes;
        use approx;

        fn actual(coeff: f32) -> Vec<f32> {
            return super::actual(1.0, coeff);
        }

        mod and_zero_coefficient {
            use super::*;
            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = original_genes();
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::*;
            #[test]
            fn entirely_changes_the_original_chromosome() {
                let actual = actual(-2.6);
                let expected = vec![-1.3635638, 1.3957189, 4.166751, 4.257335, 6.878806];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }
}