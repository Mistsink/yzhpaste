use lazy_static::lazy_static;
use parking_lot::Mutex;
use tauri::{AppHandle, GlobalShortcutManager, Manager, Window};

use crate::{cmds::open_window, utils::window_util::set_window_position_and_size};

use super::shortcuts_manager::GShortcutManager;

#[derive(Debug, Default, Clone)]
pub struct Global {
    app_handle: Option<AppHandle>
}

impl Global {
    pub fn new() -> Self {
        Global {
            app_handle: None
        }
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
        .build();
        if let Ok(window) = new_window {
            set_window_position_and_size(&window);
            return Some(window);
        }
        return None;
    }

    pub fn get_window(&self) -> (Option<Window>, bool) {
        let app = self.get_handle();
        if let Some(window) = app.get_window("main") {
            return (Some(window), false);
        }

        let new_window = self.new_window();
        if let Some(window) = new_window {
            println!("new window");
            return (Some(window), true);
        }
        return (None, false);
    }

    pub fn setup(&self) {
        GShortcutManager.register_shortcut(self.get_handle(), || _ = open_window());
    }

    pub fn exit(&self) {
        GShortcutManager.exit(self.get_handle());
    }
}

lazy_static! {
    pub static ref GLOBAL: Mutex<Global> = Mutex::new(Global::new());
}
