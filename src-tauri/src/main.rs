#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod utils;

use tauri::{Manager, window};
use utils::window::set_window_position_and_size;

fn main() {
    tauri::Builder::default()
        .on_page_load(|window, _| {
            println!("on page load");
            set_window_position_and_size(&window);
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
