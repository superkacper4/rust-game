use std::collections::HashMap;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Materials {
    pub clay: i64,
    pub grain: i64,
    pub iron: i64,
    pub stone: i64,
    pub wood: i64,
}

impl Materials {
    pub fn init_market() -> Materials {
        return Materials {
            clay: 100,
            grain: 100,
            iron: 100,
            stone: 100,
            wood: 100,
        };
    }

    pub fn init_player() -> Materials {
        return Materials {
            clay: 50,
            grain: 50,
            iron: 50,
            stone: 50,
            wood: 50,
        };
    }

    pub fn get_all_materials(&self) -> &Materials {
        return self;
    }

    pub fn add_per_tick(&mut self) -> () {
        self.wood += 5;
    }

    pub fn get_material_keys() -> [&'static str; 5] {
        return ["clay", "grain", "iron", "stone", "wood"];
    }

    pub fn get_random_materials() -> HashMap<String, i64> {
        let resources_keys = Materials::get_material_keys();
        let random_resouce_index = rand::rng().random_range(0..(resources_keys.len()));
        let mut resources = HashMap::new();
        resources.insert(resources_keys[random_resouce_index].to_string(), 5);

        return resources;
    }
}
