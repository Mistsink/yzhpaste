#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmds;
mod core;
mod setup;
mod utils;

use std::sync::Mutex;

use crate::core::{global::GLOBAL, tray};
use tauri::Manager;
use utils::window_util::set_window_position_and_size;
pub struct PreviousProcessId(Mutex<i32>);
pub struct GAppHandle(Mutex<Option<tauri::AppHandle>>);

fn main() {
    // let g_app_handle = ;

    let context = tauri::generate_context!();
    let app = tauri::Builder::default()
        .manage(GAppHandle(Mutex::new(None)))
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
            cmds::delete_older_than_days,
            cmds::write_to_clip,
            cmds::open_window,
            cmds::print,
            cmds::escape_win,
            cmds::paste_in_previous_window
        ])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .on_window_event(|event| {
            if let tauri::WindowEvent::Focused(focused) = event.event() {
                if !focused {
                    _ = cmds::escape_win();
                }
            }
        })
        .setup(move |app| setup::init(app))
        .build(context)
        .expect("error while running tauri application");
    app.run(|app, e| match e {
        tauri::RunEvent::ExitRequested { api, .. } => {
            println!("exit requested");
            api.prevent_exit();
        }
        tauri::RunEvent::Exit => {
            println!("exit");
            GLOBAL.lock().exit();
            app.exit(0);
        }
        _ => {}
    })
}
