use std::{thread};

use crate::core::{database::SqliteDB, global::GLOBAL};
use chrono::Duration;
use serde_json::json;

pub fn on_records_update() {
    let binding = GLOBAL.lock();
    let (opt_win, _) = binding.get_window();
    if let Some(window) = opt_win {
        let all_records = SqliteDB::new().find_all().unwrap();
        window
            .emit("on-records-update", json!(all_records))
            .unwrap();
    }
}

pub fn on_records_update_with_window(window: &tauri::Window) {
    let all_records = SqliteDB::new().find_all().unwrap();
    window
        .emit("on-records-update", json!(all_records))
        .unwrap();
}

pub fn on_records_update_delete() {
    let binding = GLOBAL.lock();
    // binding.stop_watcher();
    // // thread::sleep(Duration::milliseconds(1000i64).to_std().unwrap());
    // binding.start_watcher();
    let (opt_win, _) = binding.get_window();
    if let Some(window) = opt_win {
        let all_records = SqliteDB::new().find_all().unwrap();
        window
            .emit("on-records-update", json!(all_records))
            .unwrap();
    }
}

pub fn on_window_hide() {
    let binding = GLOBAL.lock();
    let (opt_win, _) = binding.get_window();
    if let Some(window) = opt_win {
        window.emit("on-window-hide", json!(null)).unwrap();
    }
}

pub fn on_window_hide_with_window(window: &tauri::Window) {
    window.emit("on-window-hide", json!(null)).unwrap();
}

pub fn on_window_show() {
    let binding = GLOBAL.lock();
    let (opt_win, _) = binding.get_window();
    if let Some(window) = opt_win {
        window.emit("on-window-show", json!(null)).unwrap();
    }
}

pub fn on_window_show_with_window(window: &tauri::Window) {
    window.emit("on-window-show", json!(null)).unwrap();
}
