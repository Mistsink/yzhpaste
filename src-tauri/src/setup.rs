use std::sync::{Arc, Mutex};

use crate::{
    core::{clipboard_listener, database::SqliteDB, global::GLOBAL},
    GAppHandle,
};
use tauri::{App, Manager, State, StateManager, AppHandle};
use window_vibrancy::{self, NSVisualEffectMaterial};

pub fn init(
    app: &mut App
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("init");

    GLOBAL.lock().init(app.app_handle());
    init_window(app);
    
    SqliteDB::init();
    clipboard_listener::ClipboardWatcher::start();
    Ok(())
}

pub fn init_window(app: &mut App) {
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    let win = app.get_window("main").unwrap();
    let _ = win.set_skip_taskbar(true);

    // 仅在 macOS 下执行
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(&win, NSVisualEffectMaterial::FullScreenUI, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // 仅在 windows 下执行
    #[cfg(target_os = "windows")]
    window_vibrancy::apply_blur(&win, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
}
