pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn new() -> Self { return Self; }

    pub fn evolve<I>(&self, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());
        return (0..population.len())
            .map(|_| {
                // TODO: selection
                // TODO: crossover
                // TODO: mutation
                todo!()
            })
            .collect();
    }
}