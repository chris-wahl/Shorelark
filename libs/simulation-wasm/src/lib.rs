use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn whos_that_dog() -> String {
    return "McGruff".into();
}


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
}


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
        }
    }
}
