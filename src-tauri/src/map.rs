use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapTile {
    value: f64,
    x: i32,
    y: i32,
}

const DIMESSIONS: i32 = 20;

#[tauri::command]
pub fn generate_map() -> Vec<MapTile> {
    // let dimessions = 20i32;
    println!("map {DIMESSIONS}x{DIMESSIONS}");

    let mut i_loop = 0i32;
    let mut map: Vec<MapTile> = Vec::new();

    loop {
        i_loop += 1;
        let mut j_loop = 0i32;

        loop {
            j_loop += 1;

            let x = i_loop;
            let y = j_loop;

            map.push(MapTile {
                value: calc_tile_value(&(x as f64), &(y as f64), &(DIMESSIONS as f64)),
                x,
                y,
            });

            if j_loop == DIMESSIONS {
                break;
            }
        }

        j_loop = 0;
        if i_loop == DIMESSIONS {
            break;
        }
    }

    return map;

    // for tile in &map {
    //     println!("{:?}", tile);
    // }
}

// (-(1/20)*(x-20)^2) + 20
fn calc_tile_value(x: &f64, y: &f64, dim: &f64) -> f64 {
    let center = dim / 2.0;

    let vx = -(1.0 / (dim * dim)) * (x - center).powi(2);
    let vy = -(1.0 / (dim * dim)) * (y - center).powi(2);

    return vx + vy + 1.0;
}
