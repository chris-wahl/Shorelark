use crate::*;

#[derive(Clone, Debug, Serialize)]
#[wasm_bindgen]
pub struct Statistics {
    pub min: f32,
    pub max: f32,
    pub avg: f32,
}

impl From<&sim::Statistics> for Statistics {
    fn from(statistics: &sim::Statistics) -> Self {
        return Self {
            min: statistics.min_fitness(),
            max: statistics.max_fitness(),
            avg: statistics.avg_fitness(),
        };
    }
}
