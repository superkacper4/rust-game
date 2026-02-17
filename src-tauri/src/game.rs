use crate::map::{self, MapTile};
use crate::player::Player;
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::Emitter;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub map: Vec<MapTile>,
    pub player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: map::generate_map(),
            player: Player::init(),
        }
    }

    pub fn buy_map_tile(tile_id: &str, game_state: &mut Game) -> Result<String, String> {
        let tile_index = game_state
            .map
            .iter()
            .position(|tile| tile.get_id() == tile_id)
            .ok_or_else(|| format!("Tile with id {} not found", tile_id))?;

        let tile = &mut game_state.map[tile_index];

        if tile.is_owned_by_player() {
            return Err("Tile is already owned by player".to_string());
        }

        let tile_value = tile.get_value();
        if game_state.player.get_cash() < tile_value {
            return Err("Not enough cash to buy this tile".to_string());
        }

        game_state.player.subtract_cash(tile_value);
        tile.set_owner_to_player();

        Ok(format!(
            "Successfully bought tile {} for ${}",
            tile_id, tile_value
        ))
    }

    pub fn tick(game_state: &mut Game, app: tauri::AppHandle) -> () {
        game_state.player.cash -= 100.0;
        app.emit("game_state_updated", &game_state).unwrap();
    }
}

#[tauri::command]
pub fn initialize_game() -> Game {
    return Game::new();
}

#[tauri::command]
pub fn buy_map_tile_command(
    tile_id: &str,
    state: tauri::State<Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let mut state = state.lock().unwrap();
    let result = Game::buy_map_tile(tile_id, &mut state.game_state);
    if result.is_ok() {
        app.emit("game_state_updated", &state.game_state).unwrap();
        Ok(format!("Successfully bought tile {}", tile_id))
    } else {
        Err("Something went wrong".to_string())
    }
}

#[tauri::command]
pub fn get_game_state(state: tauri::State<Mutex<AppState>>) -> Game {
    let state = state.lock().unwrap();
    state.game_state.clone()
}

#[tauri::command]
pub fn end_turn(
    state: tauri::State<Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let mut state = state.lock().unwrap();
    Game::tick(&mut state.game_state, app);
    return Ok("Turn ended".to_string());
}
