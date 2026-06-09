use crate::map::{self, get_build_cost, BuildingKind, MapTile};
use crate::materials::Materials;
use crate::player::{Player, PlayerId};
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::Emitter;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub current_player_turn: PlayerId,
    pub map: Vec<MapTile>,
    pub market: Materials,
    pub players: HashMap<PlayerId, Player>,
    pub players_queue: Vec<PlayerId>,
}

impl Game {
    pub fn new() -> Self {
        let mut players_queue = Vec::new();
        let mut players = HashMap::new();

        players.insert(
            PlayerId::Player,
            Player::init(PlayerId::Player, "Kacper".to_string()),
        );
        players.insert(
            PlayerId::Enemy,
            Player::init(PlayerId::Enemy, "Johnny".to_string()),
        );

        players_queue.push(PlayerId::Enemy);
        players_queue.push(PlayerId::Player);

        Self {
            current_player_turn: PlayerId::Player,
            map: map::generate_map(),
            market: Materials::init_market(),
            players,
            players_queue,
        }
    }

    pub fn buy_map_tile(tile_id: &str, game_state: &mut Game) -> Result<String, String> {
        let player = game_state
            .players
            .get_mut(&game_state.current_player_turn)
            .expect("No player with given ID");

        if player.check_if_out_of_actions_left() {
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

        if player.get_cash() < tile_value {
            return Err("Not enough cash to buy this tile".to_string());
        }

        player.subtract_cash(tile_value);
        tile.set_owner_to_player(&game_state.current_player_turn);
        if let Err(err) = player.take_one_action() {
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
        let player = game_state
            .players
            .get_mut(&game_state.current_player_turn)
            .expect("No player with given ID");

        if player.check_if_out_of_actions_left() {
            return Err("No actions left in this turn".to_string());
        }

        let build_cost_materials = get_build_cost(building_kind);

        if !player.check_if_can_afford(build_cost_materials) {
            return Err("Not enough materials to build".to_string());
        }
        player.subtract_materials(build_cost_materials);

        let map_tile_index = game_state
            .map
            .iter()
            .position(|tile| tile.get_id() == tile_id)
            .ok_or_else(|| format!("Tile with id {} not found", tile_id))?;

        MapTile::change_building(&mut game_state.map[map_tile_index], building_kind);

        if let Err(err) = player.take_one_action() {
            eprintln!("Soft fail: {}", err);
        }
        app.emit("game_state_updated", &game_state).unwrap();
        Ok("Building's been changed.".to_string())
    }

    pub fn get_next_turn_player(&self) -> PlayerId {
        let index_of_current_player = self
            .players_queue
            .iter()
            .position(|player_id| player_id == &self.current_player_turn)
            .expect("No player in the queue.");

        let length_of_queue = self.players_queue.len();

        if index_of_current_player == length_of_queue - 1 {
            return self.players_queue[0];
        }

        return self.players_queue[index_of_current_player + 1];
    }

    pub fn sell_map_tile(tile_id: &str, game_state: &mut Game) -> Result<String, String> {
        let player = game_state
            .players
            .get_mut(&game_state.current_player_turn)
            .expect("No player with given ID");

        if player.take_one_action().is_err() {
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
        player.add_cash(tile_clone.get_value());

        return Ok("MapTile has been sold".to_string());
    }

    pub fn tick(game_state: &mut Game, app: tauri::AppHandle) -> () {
        let player = game_state
            .players
            .get_mut(&game_state.current_player_turn)
            .expect("No player with given ID");

        let tiles_owned = game_state
            .map
            .iter()
            .filter(|x| x.is_owned_by_player(&game_state.current_player_turn))
            .collect();

        player.materials = player.change_materials(&tiles_owned);

        let number_of_tiles_owned = tiles_owned.len() as i64;
        player.cash -= (1 + number_of_tiles_owned) * 100;

        player.actions_left_in_turn = 2;
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
