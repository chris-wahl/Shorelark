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
            chromosome.iter_mut().for_each(| gene| *gene *= multiplier);

            let genes: Vec<_> = chromosome.iter().collect();
            let known = known_genes();
            for i in 0..known.len() {
                assert_eq!(genes[i], &(known[i] * multiplier));
            }

        }
    }
}