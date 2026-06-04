use crate::map::{self, get_build_cost, BuildingKind, MapTile};
use crate::materials::Materials;
use crate::player::Player;
use crate::{game, AppState};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::Emitter;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub current_player_turn: String,
    pub map: Vec<MapTile>,
    pub market: Materials,
    pub player: Player,
    pub enemy1: Player,
    pub players_queue: Vec<String>,
}

impl Game {
    pub fn new() -> Self {
        let mut players_queue = Vec::new();
        let player_id = "player";
        let enemy_id = "enemy1";

        let player = Player::init(&player_id);
        let enemy1 = Player::init(&enemy_id);

        players_queue.push(player_id.to_string());
        players_queue.push(enemy_id.to_string());
        Self {
            current_player_turn: player_id.to_string(),
            enemy1,
            map: map::generate_map(),
            market: Materials::init_market(),
            player,
            players_queue,
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

        if tile.is_owned() {
            return Err("Tile is already owned".to_string());
        }

        let tile_value = tile.get_value();
        let player = game_state.get_current_player_mut();
        if game_state.player.get_cash() < tile_value {
            return Err("Not enough cash to buy this tile".to_string());
        }

        game_state.player.subtract_cash(tile_value);
        tile.set_owner_to_player(&game_state.current_player_turn);
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

    pub fn get_current_player_mut(&mut self) -> &mut Player {
        match self.current_player_turn.as_str() {
            "player" => &mut self.player,
            "enemy1" => &mut self.enemy1,
            _ => panic!("Unknown player"),
        }
    }

    pub fn get_next_turn_player(&self) -> String {
        match self.current_player_turn.as_str() {
            "player" => "enemy1".to_string(),
            "enemy1" => "player".to_string(),
            _ => panic!("Unknown player"),
        }
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
        MapTile::set_owner_to_game(tile);

        let tile_clone = tile.clone();
        let current_player = game_state.get_current_player_mut();
        current_player.add_cash(tile_clone.get_value());

        return Ok("MapTile has been sold".to_string());
    }

    pub fn tick(game_state: &mut Game, app: tauri::AppHandle) -> () {
        let game_state_clone = game_state.clone();
        let tiles_owned = game_state_clone
            .map
            .iter()
            .filter(|x| x.is_owned_by_player(&game_state.current_player_turn))
            .collect();

        let current_player = game_state.get_current_player_mut();

        current_player.materials = current_player.change_materials(&tiles_owned);

        let number_of_tiles_owned = tiles_owned.len() as i64;
        current_player.cash -= (1 + number_of_tiles_owned) * 100;

        current_player.actions_left_in_turn = 2;
        game_state.market.add_per_tick();
        game_state.current_player_turn = game_state.get_next_turn_player();
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
