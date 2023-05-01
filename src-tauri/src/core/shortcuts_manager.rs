use tauri::{AppHandle, GlobalShortcutManager};
#[derive(Debug, Default, Clone)]
pub struct GShortcutManager;
impl GShortcutManager {
    fn get_shortcuts_manager(&self, app: &AppHandle) -> impl GlobalShortcutManager {
        app.global_shortcut_manager()
    }
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
