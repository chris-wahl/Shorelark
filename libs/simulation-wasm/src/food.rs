use crate::*;

#[derive(Clone, Debug, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        return Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}