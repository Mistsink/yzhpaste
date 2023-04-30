use lazy_static::lazy_static;
use parking_lot::Mutex;
use tauri::{AppHandle};
pub struct Global {
    app_handle: Option<AppHandle>,
}

impl Global {
    pub fn new() -> Self {
        Global { app_handle: None }
    }

    pub fn init(&mut self, app_handle: AppHandle) {
        self.app_handle = Some(app_handle)
    }

    pub fn get_handle(&self) -> Option<&AppHandle> {
        // self.app_handle.as_ref().unwrap()
        self.app_handle.as_ref()
    }
}

lazy_static! {
    pub static ref GLOBAL: Mutex<Global> = Mutex::new(Global::new());
}