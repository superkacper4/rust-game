use crate::game::Game;
use std::sync::Mutex;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct AppState {
    game_state: Game,
}

mod game;
mod map;
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
            greet,
            game::buy_map_tile_command,
            game::get_game_state,
            game::initialize_game,
            game::end_turn
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
