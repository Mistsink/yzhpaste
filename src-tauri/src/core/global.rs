use chrono::Local;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use tauri::{AppHandle, Manager, Window};

use crate::utils::window_util::{set_window_position_and_size, ProcessInfo};

use super::shortcuts_manager::GShortcutManager;

#[derive(Debug, Default, Clone)]
pub struct Global {
    app_handle: Option<AppHandle>,
    pre_process_id: i32,
    pre_process_info: ProcessInfo,
}

impl Global {
    pub fn new() -> Self {
        Global {
            app_handle: None,
            pre_process_id: 0,
            pre_process_info: ProcessInfo::default(),
        }
    }

    pub fn get_pre_process_info(&self) -> ProcessInfo {
        return self.pre_process_info.clone();
    }

    pub fn set_pre_process_info(&mut self, info: ProcessInfo) {
        println!("set_pre_process_info: {:?}", info);
        self.pre_process_info = info;
    }

    pub fn init(&mut self, app_handle: AppHandle) {
        self.app_handle = Some(app_handle)
    }

    pub fn get_handle(&self) -> &AppHandle {
        self.app_handle.as_ref().unwrap()
    }

    fn new_window(&self) -> Option<Window> {
        let app = self.get_handle();

        let new_window = tauri::window::WindowBuilder::new(
            app,
            "main".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .title("yzhpaste")
        .visible(false)
        .resizable(false)
        .fullscreen(false)
        .transparent(true)
        .decorations(false)
        .skip_taskbar(true)
        .focused(true)
        .build();
        if let Ok(window) = new_window {
            set_window_position_and_size(&window);
            return Some(window);
        }
        return None;
    }

    pub fn get_window(&self) -> (Option<Window>, bool) {
        println!("[{}] in GLOBAL get_window", Local::now());

        let app = self.get_handle();
        if let Some(window) = app.get_window("main") {
            println!("[{}] out GLOBAL get_window", Local::now());
            return (Some(window), false);
        }

        let new_window = self.new_window();
        if let Some(window) = new_window {
            println!("new window");
            println!("[{}] out GLOBAL get_window", Local::now());
            return (Some(window), true);
        }
        println!("[{}] out GLOBAL get_window", Local::now());
        return (None, false);
    }

    pub fn load_shortcut_manager(&self) {
        GShortcutManager.load_cfg(self.get_handle());
    }

    pub fn exit(&self) {
        GShortcutManager.exit(self.get_handle());
    }
}

lazy_static! {
    pub static ref GLOBAL: Mutex<Global> = Mutex::new(Global::new());
}
