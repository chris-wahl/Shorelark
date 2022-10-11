
use serde::Serialize;
use lib_simulation as sim;
use crate::animal::*;

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub animals: Vec<Animal>,
}


impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world
            .animals()
            .iter()
            .map(Animal::from)
            .collect();
        return Self { animals };
    }
}
