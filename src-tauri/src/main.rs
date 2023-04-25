#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod plugins;
mod setup;
mod tray;
mod utils;

use utils::window::set_window_position_and_size;
use plugins::clipboard_listener::get_clipboard_data;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_clipboard_data])
    .menu(tauri::Menu::os_default(&context.package_info().name))
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .on_page_load(|window, _| {
            println!("on page load");
            set_window_position_and_size(&window);
        })
        .setup(setup::init)
        .run(context)
        .expect("error while running tauri application");
}
