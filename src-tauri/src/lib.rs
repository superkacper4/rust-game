use crate::game::Game;
use std::sync::Mutex;
use tauri::Manager;

struct AppState {
    game_state: Game,
}

mod game;
mod map;
mod materials;
mod player;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState {
                game_state: Game::new(),
            }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            game::buy_map_tile_command,
            game::change_building_for_tile,
            game::get_game_state,
            game::initialize_game,
            game::end_turn,
            game::sell_map_tile_command,
            map::check_if_player_owns_tile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
