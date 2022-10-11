use serde::Serialize;
use lib_simulation as sim;

#[derive(Clone, Debug, Serialize)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        return Self {
            x: animal.position().x,
            y: animal.position().y,
        };
    }
}
