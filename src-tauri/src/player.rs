use serde::{Deserialize, Serialize};

use crate::{map::MapTile, materials::Materials};

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: String,
    pub actions_left_in_turn: i32,
    pub cash: i64, // cash in cents
    pub name: String,
    pub materials: Materials,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PlayerId {
    Player,
    Enemy,
}

impl Player {
    pub fn add_cash(&mut self, amount: i64) {
        self.cash += amount;
    }

    pub fn change_materials(&mut self, map_tiles: &Vec<&MapTile>) -> Materials {
        let player_materials = self.materials;
        return map_tiles.iter().fold(player_materials, |acc, tile| {
            let material = tile.get_resource();

            return Materials {
                clay: acc.clay + material.get("clay").unwrap_or(&0),
                grain: acc.grain + material.get("grain").unwrap_or(&0),
                iron: acc.iron + material.get("iron").unwrap_or(&0),
                stone: acc.stone + material.get("stone").unwrap_or(&0),
                wood: acc.wood + material.get("wood").unwrap_or(&0),
            };
        });
    }

    pub fn check_if_out_of_actions_left(&self) -> bool {
        if self.actions_left_in_turn > 0 {
            return false;
        }
        return true;
    }

    pub fn check_if_can_afford(&self, materials: Materials) -> bool {
        return self.materials.clay >= materials.clay
            && self.materials.grain >= materials.grain
            && self.materials.iron >= materials.iron
            && self.materials.stone >= materials.stone
            && self.materials.wood >= materials.wood;
    }

    pub fn take_one_action(&mut self) -> Result<String, String> {
        if self.check_if_out_of_actions_left() {
            return Err("Not enough actions left".to_string());
        }
        self.actions_left_in_turn -= 1;
        Ok("Action deleted sucessfully".to_string())
    }

    pub fn get_cash(&self) -> i64 {
        self.cash
    }

    pub fn subtract_cash(&mut self, amount: i64) {
        self.cash -= amount;
    }

    pub fn subtract_materials(&mut self, materials: Materials) {
        self.materials.clay -= materials.clay;
        self.materials.grain -= materials.grain;
        self.materials.iron -= materials.iron;
        self.materials.stone -= materials.stone;
        self.materials.wood -= materials.wood;
    }

    pub fn init(id: &str) -> Player {
        return Player {
            id: id.to_string(),
            actions_left_in_turn: 2,
            cash: 10000000,
            name: "Kacper".to_owned(),
            materials: Materials::init_player(),
        };
    }
}
