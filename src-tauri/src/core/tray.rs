use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use super::global::GLOBAL;

// 托盘菜单
pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("hide".to_string(), "Hide")) // 隐藏应用窗口
        .add_item(CustomMenuItem::new("show".to_string(), "Show")) // 显示应用窗口
        .add_native_item(SystemTrayMenuItem::Separator) // 分割线
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit")); // 退出

    // 设置在右键单击系统托盘时显示菜单
    SystemTray::new().with_menu(tray_menu)
}

// 菜单事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    // 获取应用窗口
    // let mut opt_window: Option<Window> = None;
    // // {
    let (opt_window, _) = GLOBAL.lock().get_window();
    // }
    if let Some(window) = opt_window {
        match event {
            // 左键点击
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            // 右键点击
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            // 根据菜单 id 进行事件匹配
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    app.exit(0);
                    std::process::exit(0);
                }
                "show" => {
                    window.show().unwrap();
                }
                "hide" => {
                    window.hide().unwrap();
                }
                _ => {}
            },
            _ => {}
        }
    }

    // 匹配点击事件
}
