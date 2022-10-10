#![feature(type_alias_impl_trait)]

use std::ops::Index;

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        return self.genes.len();
    }

    pub fn iter(&self) -> impl Iterator<Item=&f32> {
        return self.genes.iter();
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut f32> {
        return self.genes.iter_mut();
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.genes[index];
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item=f32>>(iter: T) -> Self {
        return Self { genes: iter.into_iter().collect() };
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        return self.genes.into_iter();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn known_genes() -> Vec<f32> {
        return vec![3.0, 1.0, 2.0];
    }

    fn chromosome() -> Chromosome {
        return Chromosome {
            genes: known_genes()
        };
    }

    mod len {
        use super::*;

        #[test]
        fn test() {
            assert_eq!(chromosome().len(), known_genes().len());
        }
    }

    mod iter {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();
            let genes: Vec<_> = chromosome.iter().collect();

            let known = known_genes();
            assert_eq!(genes.len(), known.len());
            for i in 0..known.len() {
                assert_eq!(genes[i], &known[i]);
            }
        }
    }

    mod iter_mut {
        use super::*;

        #[test]
        fn test() {
            let mut chromosome = chromosome();

            let multiplier = 10.0;
            chromosome.iter_mut().for_each(|gene| *gene *= multiplier);

            let genes: Vec<_> = chromosome.iter().collect();
            let known = known_genes();
            for i in 0..known.len() {
                assert_eq!(genes[i], &(known[i] * multiplier));
            }
        }
    }

    mod index {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();
            let known = known_genes();
            for i in 0..known.len() {
                assert_eq!(chromosome[i], known[i]);
            }
        }
    }

    mod from_iterator {
        use super::*;

        #[test]
        fn test() {
            let known = known_genes();
            let chromosome: Chromosome = known.clone().into_iter().collect();

            for i in 0..known.len() {
                assert_eq!(chromosome[i], known[i]);
            }
        }
    }

    mod into_iterator {
        use super::*;

        #[test]
        fn test() {
            let known = known_genes();
            let chromosome = chromosome();

            let genes: Vec<_> = chromosome.into_iter().collect();

            for i in 0..known.len() {
                assert_eq!(genes[i], known[i]);
            }
        }
    }
}