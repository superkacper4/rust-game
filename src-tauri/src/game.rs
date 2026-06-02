use crate::map::{self, get_build_cost, BuildingKind, MapTile};
use crate::materials::Materials;
use crate::player::{self, Player};
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::Emitter;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub map: Vec<MapTile>,
    pub market: Materials,
    pub player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: map::generate_map(),
            market: Materials::init_market(),
            player: Player::init(),
        }
    }

    pub fn buy_map_tile(tile_id: &str, game_state: &mut Game) -> Result<String, String> {
        if game_state.player.check_if_out_of_actions_left() {
            return Err("No actions left in this turn".to_string());
        }

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
        if let Err(err) = game_state.player.take_one_action() {
            eprintln!("Soft fail: {}", err);
        }

        Ok(format!(
            "Successfully bought tile {} for ${}",
            tile_id, tile_value
        ))
    }

    pub fn change_building(
        tile_id: &str,
        building_kind: BuildingKind,
        game_state: &mut Game,
        app: tauri::AppHandle,
    ) -> Result<String, String> {
        if game_state.player.check_if_out_of_actions_left() {
            return Err("No actions left in this turn".to_string());
        }

        let build_cost_materials = get_build_cost(building_kind);

        if !game_state.player.check_if_can_afford(build_cost_materials) {
            return Err("Not enough materials to build".to_string());
        }
        game_state.player.subtract_materials(build_cost_materials);

        let map_tile_index = game_state
            .map
            .iter()
            .position(|tile| tile.get_id() == tile_id)
            .ok_or_else(|| format!("Tile with id {} not found", tile_id))?;

        MapTile::change_building(&mut game_state.map[map_tile_index], building_kind);

        if let Err(err) = game_state.player.take_one_action() {
            eprintln!("Soft fail: {}", err);
        }
        app.emit("game_state_updated", &game_state).unwrap();
        Ok("Building's been changed.".to_string())
    }

    pub fn sell_map_tile(tile_id: &str, game_state: &mut Game) -> Result<String, String> {
        if game_state.player.take_one_action().is_err() {
            return Err("No actions left in this turn".to_string());
        }

        let tile_index = game_state
            .map
            .iter()
            .position(|tile| tile.get_id() == tile_id)
            .ok_or_else(|| format!("Tile with id {} not found", tile_id))?;

        let tile = &mut game_state.map[tile_index];

        game_state.player.add_cash(tile.get_value());
        MapTile::set_owner_to_game(tile);

        return Ok("MapTile has been sold".to_string());
    }

    pub fn tick(game_state: &mut Game, app: tauri::AppHandle) -> () {
        let tiles_owned = game_state
            .map
            .iter()
            .filter(|x| x.is_owned_by_player())
            .collect();

        game_state.player.materials = game_state.player.change_materials(&tiles_owned);

        let number_of_tiles_owned = tiles_owned.len() as i64;
        game_state.player.cash -= (1 + number_of_tiles_owned) * 100;

        game_state.player.actions_left_in_turn = 2;
        game_state.market.add_per_tick();
        app.emit("game_state_updated", &game_state).unwrap();
    }
}

#[tauri::command]
pub fn initialize_game(state: tauri::State<Mutex<AppState>>, app: tauri::AppHandle) -> Game {
    let mut state = state.lock().unwrap();
    let game = Game::new();
    state.game_state = game.clone();
    app.emit("game_state_updated", &state.game_state).unwrap();
    return game;
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
pub fn sell_map_tile_command(
    tile_id: &str,
    state: tauri::State<Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let mut state = state.lock().unwrap();
    let result = Game::sell_map_tile(tile_id, &mut state.game_state);
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
pub fn change_building_for_tile(
    tile_id: &str,
    building_kind: BuildingKind,
    state: tauri::State<Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let mut state = state.lock().unwrap();
    return Game::change_building(tile_id, building_kind, &mut state.game_state, app);
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
