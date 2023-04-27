#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmds;
mod core;
mod setup;
mod utils;

use crate::core::{tray};
use utils::window_util::set_window_position_and_size;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmds::get_clipboard_data,
            cmds::clear_data,
            cmds::insert_record,
            cmds::insert_if_not_exist,
            cmds::find_all_records,
            cmds::mark_pined,
            cmds::save_tags,
            cmds::find_by_key,
            cmds::delete_over_limit,
            cmds::delete_by_id,
            cmds::delete_older_than_days
            // cmds::write_to_clip,
        ])
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
