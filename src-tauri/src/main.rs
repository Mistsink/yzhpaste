#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmds;
mod config;
mod core;
mod events;
use chrono::{DateTime, Local, Duration};
use events::on_records_update;
mod setup;
mod utils;
use crate::core::{global::GLOBAL, tray};
use std::sync::{Arc, Mutex};
pub struct PreviousProcessId(Mutex<i32>);
pub struct GAppHandle(Mutex<Option<tauri::AppHandle>>);

fn main() {
    // let g_app_handle = ;
    let app_state = AppState::new();

    let context = tauri::generate_context!();
    let app = tauri::Builder::default()
        .manage(GAppHandle(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            // config
            cmds::get_common_config,
            cmds::set_common_config,
            cmds::change_language,
            cmds::change_record_limit,
            cmds::change_auto_launch,
            cmds::change_hotkeys,
            cmds::change_delete_confirm,
            cmds::change_theme_mode,
            cmds::change_auto_paste,
            // record
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
        .on_window_event(move |event| {
            let last_focus_time = Arc::clone(&app_state.last_focus_time);
            // if let tauri::WindowEvent::Focused(focused) = event.event() {
            //     println!("window focused: {} [{}]", focused, Local::now());
            //     if let Ok(mut last_focus_time) = last_focus_time.lock() {
            //         if *focused {
            //             *last_focus_time = Local::now();
            //         } else {
            //             let delta_duration = 100; // 毫秒
            //             let now = Local::now();
            //             if now - *last_focus_time > Duration::milliseconds(delta_duration) {
            //                 _ = cmds::escape_win();
            //             }
            //         }
            //     }
            // }
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
    });
}

struct AppState {
    last_focus_time: Arc<Mutex<DateTime<Local>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            last_focus_time: Arc::new(Mutex::new(Local::now())), // 初始为当前时间
        }
    }
}
