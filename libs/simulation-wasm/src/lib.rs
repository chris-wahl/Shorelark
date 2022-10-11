use wasm_bindgen::prelude::*;

use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;

pub use crate::{
    animal::*,
    food::*,
    world::*,
};

mod animal;
mod food;
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
        return JsValue::from_serde(&world).unwrap();
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }
}
