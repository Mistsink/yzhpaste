use std::sync::{Arc, Mutex};

use crate::{
    cmds::open_window,
    core::{clipboard, database::SqliteDB, global::GLOBAL},
    utils::{dispatch_util::request_permissions, window_util::get_active_process_id},
    GAppHandle,
};
use tauri::{window, App, AppHandle, Manager, State, StateManager, Window};
use window_vibrancy::{self, NSVisualEffectMaterial};

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("init");

    {
        GLOBAL.lock().init(app.app_handle());
        GLOBAL.lock().setup();
        GLOBAL.lock().get_window();
    }
    SqliteDB::init();
    clipboard::ClipboardWatcher::start();

    init_window(app);

    Ok(())
}

pub fn init_window(app: &mut App) {
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    let opt_win: Option<Window>;
    {
        let mut binding = GLOBAL.lock();
        binding.set_pre_process_id(get_active_process_id());
        (opt_win, _) = binding.get_window();
    }

    if let Some(win) = opt_win {
        // 仅在 macOS 下执行
        #[cfg(target_os = "macos")]
        window_vibrancy::apply_vibrancy(&win, NSVisualEffectMaterial::FullScreenUI, None, None)
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

        // 仅在 windows 下执行
        #[cfg(target_os = "windows")]
        window_vibrancy::apply_blur(&win, Some((18, 18, 18, 125)))
            .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

        request_permissions();
        _ = win.show();
    }
}
