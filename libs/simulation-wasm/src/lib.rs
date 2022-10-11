use rand::prelude::*;
use wasm_bindgen::prelude::*;
use serde::Serialize;

use lib_simulation as sim;

pub use crate::{
    animal::*,
    food::*,
    statistics::*,
    world::*,
};

mod animal;
mod food;
mod statistics;
mod world;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        return Self { rng, sim };
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        return serde_wasm_bindgen::to_value(&world).unwrap();
    }

    pub fn step(&mut self) -> JsValue {
        if let Some(statistics) = self.sim.step(&mut self.rng) {
            return serde_wasm_bindgen::to_value(&Statistics::from(&statistics)).unwrap();
        }
        return JsValue::null();
    }

    pub fn train(&mut self) -> JsValue {
        let stats = self.sim.train(&mut self.rng);
        return serde_wasm_bindgen::to_value(&Statistics::from(&stats)).unwrap();
    }
}
