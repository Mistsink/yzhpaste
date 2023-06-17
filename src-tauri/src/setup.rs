use crate::{
    cmds,
    config::Config,
    core::{clipboard, database::SqliteDB, global::GLOBAL, sysopt},
    log_err,
    utils::{dispatch_util::request_permissions, window_util::get_active_process_info},
};
use tauri::{App, Manager, Window};
use window_vibrancy::{self, NSVisualEffectMaterial};

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    log_err!(Config::init_config()); //  first

    {
        GLOBAL.lock().init(app.app_handle());
        GLOBAL.lock().load_shortcut_manager();
        GLOBAL.lock().get_window();
    }
    SqliteDB::init();
    clipboard::ClipboardWatcher::start();
    log_err!(sysopt::Sysopt::global().init_launch());

    init_window(app);

    Ok(())
}

pub fn init_window(app: &mut App) {
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    let opt_win: Option<Window>;
    {
        let mut binding = GLOBAL.lock();
        binding.set_pre_process_info(get_active_process_info());
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

        _ = win.show();
        request_permissions();
    }
}
