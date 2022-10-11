use crate::{Animal, Food, RngCore};

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40)
            .map(|_| Animal::random(rng))
            .collect();

        let foods = (0..60)
            .map(|_| Food::random(rng))
            .collect();

        return Self { animals, foods };
    }
    pub fn animals(&self) -> &[Animal] {
        return &self.animals;
    }
    pub fn foods(&self) -> &[Food] {
        return &self.foods;
    }
}
