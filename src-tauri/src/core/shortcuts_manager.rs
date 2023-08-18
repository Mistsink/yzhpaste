use tauri::{AppHandle, GlobalShortcutManager};

use crate::{cmds::open_window, config::Config};
#[derive(Debug, Default, Clone)]
pub struct GShortcutManager;
impl GShortcutManager {
    fn get_shortcuts_manager(&self, app: &AppHandle) -> impl GlobalShortcutManager {
        app.global_shortcut_manager()
    }
    pub fn load_cfg(&self, app: &AppHandle) {
        let hotkeys = Config::common().latest().hotkeys.clone();
        if let Some(hotkeys) = hotkeys {
            let mut manager = self.get_shortcuts_manager(app);
            _ = manager.unregister_all();
            
            println!("hotkeys: {:?}", hotkeys);

            
            // manager.register(hotkeys.active.as_str(), || {
            //     _ = open_window();
            // });
            if let Err(e) = manager.register(hotkeys.active.as_str(), || _ = open_window()) {
                println!("register active hotkey error: {:?}", e);
            }
        }
    }
    #[allow(unused)]
    pub fn register_shortcut(&self, app: &AppHandle, callback: impl Fn() + Send + 'static) {
        let mut manager = self.get_shortcuts_manager(app);
        _ = manager.register("CmdOrCtrl+Shift+C", move || {
            callback();
        });
    }
    pub fn unregister_all(&self, app: &AppHandle) {
        let mut manager = self.get_shortcuts_manager(app);
        _ = manager.unregister_all();
    }

    pub fn exit(&self, app: &AppHandle) {
        self.unregister_all(app);
    }
}
