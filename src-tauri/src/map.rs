use std::sync::Mutex;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::AppState;

const DIMESSIONS: i64 = 20;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum BuildingKind {
    APARTMENT,
    EMPTY,
    FACTORY,
    FARM,
    HOUSE,
    OFFICE,
    SHOP,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum OwnerKind {
    Player,
    Game,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MapTile {
    building: BuildingKind,
    id: String,
    owner: OwnerKind,
    x: i64,
    y: i64,
    value: i64,
}

impl MapTile {
    pub fn change_building(&mut self, new_building: BuildingKind) -> () {
        self.building = new_building;
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn is_owned_by_player(&self) -> bool {
        matches!(self.owner, OwnerKind::Player)
    }

    pub fn set_owner_to_player(&mut self) {
        self.owner = OwnerKind::Player;
    }

    pub fn set_owner_to_game(&mut self) {
        self.owner = OwnerKind::Game;
    }
}

pub fn generate_map() -> Vec<MapTile> {
    let mut i_loop = 0i64;
    let mut map: Vec<MapTile> = Vec::new();
    let random_high_value_center = pick_random_high_value_center();

    loop {
        i_loop += 1;
        let mut j_loop = 0i64;

        loop {
            j_loop += 1;

            let x = i_loop;
            let y = j_loop;

            let value = calc_tile_value(&x, &y, &DIMESSIONS, &random_high_value_center);

            map.push(MapTile {
                building: get_default_building(value),
                id: format!("{}{}", x, y),
                owner: OwnerKind::Game,
                value,
                x,
                y,
            });

            if j_loop == DIMESSIONS {
                break;
            }
        }

        if i_loop == DIMESSIONS {
            break;
        }
    }

    return map;
}

// (-(1/20)*(x-20)^2) + 20
fn calc_tile_value(x: &i64, y: &i64, dim: &i64, &random_high_value_center: &f64) -> i64 {
    let center = dim / 2;

    // Trzeba zmieszać tutaj kilka funkcji np. w 5x5 mamy x^4, a potem już x^2, potem jak jest w obszarze random_high_value_center, to też inaczej itd.

    return -(x - center).pow(2) - (y - center).pow(2) + dim.pow(2);
}

fn get_default_building(value: i64) -> BuildingKind {
    let max = (DIMESSIONS * DIMESSIONS) as f64;
    let value_f = value as f64;

    match value_f {
        v if v >= max * 0.985 => BuildingKind::OFFICE,
        v if v >= max * 0.92 => {
            [BuildingKind::APARTMENT, BuildingKind::OFFICE][rand::rng().random_range(0..2)]
        }
        v if v >= max * 0.89 => [
            BuildingKind::APARTMENT,
            BuildingKind::HOUSE,
            BuildingKind::SHOP,
        ][rand::rng().random_range(0..3)],
        v if v >= max * 0.85 => {
            [BuildingKind::HOUSE, BuildingKind::SHOP][rand::rng().random_range(0..2)]
        }
        v if v >= max * 0.80 => [
            BuildingKind::FACTORY,
            BuildingKind::FARM,
            BuildingKind::HOUSE,
            BuildingKind::EMPTY,
        ][rand::rng().random_range(0..4)],
        _ => BuildingKind::EMPTY,
    }
}

fn pick_random_high_value_center() -> f64 {
    let num = rand::rng().random_range(0..(DIMESSIONS / 4));

    return num as f64;
}

#[tauri::command]
pub fn check_if_player_owns_tile(tile_id: &str, state: tauri::State<Mutex<AppState>>) -> bool {
    let state = state.lock().unwrap();

    let tile_index = state
        .game_state
        .map
        .iter()
        .position(|tile| tile.id == tile_id)
        .ok_or_else(|| format!("Tile with id {} not found", tile_id));

    let tile_index = match tile_index {
        Ok(index) => index,
        Err(_) => return false,
    };

    return state.game_state.map[tile_index].is_owned_by_player();
}
