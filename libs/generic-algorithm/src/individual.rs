use crate::Chromosome;

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        return Self { fitness };
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        return self.fitness;
    }

    fn chromosome(&self) -> &Chromosome {
        panic!("not supported for TestIndividual");
    }
}
