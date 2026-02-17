use rand::Rng;
use serde::{Deserialize, Serialize};

const DIMESSIONS: i32 = 20;

#[derive(Serialize, Deserialize, Clone)]
pub enum OwnerKind {
    Player,
    Game,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MapTile {
    id: String,
    owner: OwnerKind,
    x: i32,
    y: i32,
    value: f64,
}

impl MapTile {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn is_owned_by_player(&self) -> bool {
        matches!(self.owner, OwnerKind::Player)
    }

    pub fn set_owner_to_player(&mut self) {
        self.owner = OwnerKind::Player;
    }
}

pub fn generate_map() -> Vec<MapTile> {
    let mut i_loop = 0i32;
    let mut map: Vec<MapTile> = Vec::new();
    let random_high_value_center = pick_random_high_value_center();

    loop {
        i_loop += 1;
        let mut j_loop = 0i32;

        loop {
            j_loop += 1;

            let x = i_loop;
            let y = j_loop;

            map.push(MapTile {
                id: format!("{}{}", x, y),
                owner: OwnerKind::Game,
                value: calc_tile_value(
                    &(x as f64),
                    &(y as f64),
                    &(DIMESSIONS as f64),
                    &random_high_value_center,
                ),
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
fn calc_tile_value(x: &f64, y: &f64, dim: &f64, &random_high_value_center: &f64) -> f64 {
    let center = dim / 2.0;

    // Trzeba zmieszać tutaj kilka funkcji np. w 5x5 mamy x^4, a potem już x^2, potem jak jest w obszarze random_high_value_center, to też inaczej itd.

    return -(x - center).powi(2) - (y - center).powi(2) + dim.powi(2);
}

fn pick_random_high_value_center() -> f64 {
    let num = rand::rng().random_range(0..(DIMESSIONS / 4));

    return num as f64;
}
