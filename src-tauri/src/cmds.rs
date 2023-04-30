use tauri::{AppHandle, Manager, State};

use crate::{
    core::{
        clipboard_listener,
        database::{QueryReq, Record, SqliteDB},
        global::GLOBAL,
    },
    utils::{dispatch_util, window_util::focus_window},
    GAppHandle, PreviousProcessId,
};

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub async fn get_clipboard_data() -> CmdResult<String> {
    clipboard_listener::get_clipboard_data().await
}

#[tauri::command]
pub fn clear_data() -> bool {
    match SqliteDB::new().clear_data() {
        Ok(()) => true,
        Err(_) => false,
    }
}

#[tauri::command]
pub fn insert_record(r: Record) -> bool {
    match SqliteDB::new().insert_record(r) {
        Ok(_i) => true,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn insert_if_not_exist(r: Record) -> bool {
    match SqliteDB::new().insert_if_not_exist(r) {
        Ok(_i) => true,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn find_all_records() -> Vec<Record> {
    SqliteDB::new().find_all().unwrap()
}

#[tauri::command]
pub fn mark_pined(id: u64) -> bool {
    match SqliteDB::new().mark_pined(id) {
        Ok(_i) => true,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn save_tags(id: u64, tags: String) -> bool {
    match SqliteDB::new().save_tags(id, tags) {
        Ok(_i) => true,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn delete_by_id(id: u64) -> bool {
    match SqliteDB::new().delete_by_id(id) {
        Ok(_i) => true,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn find_by_key(query: QueryReq) -> Vec<Record> {
    SqliteDB::new().find_by_key(query).unwrap()
}

#[tauri::command]
pub fn delete_over_limit(limit: usize) -> bool {
    match SqliteDB::new().delete_over_limit(limit) {
        Ok(res) => res,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn delete_older_than_days(days: i64) -> bool {
    match SqliteDB::new().delete_older_than_days(days) {
        Ok(res) => res,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn open_window() -> CmdResult {
    println!("open window");
    // GLOBAL.lock()
    let binding = GLOBAL.lock();
    let app_handle = binding.get_handle();
    if let Some(app) = app_handle {
        if let Some(window) = app.get_window("main") {
            if window.is_visible().unwrap() {
                let _ = window.close();
                return Ok(());
            }
            let _ = window.unminimize();
            _ = window.show();
            _ = window.set_focus();
            return Ok(());
        }

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
        .decorations(true)
        .skip_taskbar(true)
        .build();

        match new_window {
            Ok(window) => {
                println!("new window");
                let _ = window.show();
                let _ = window.set_focus();
            }
            Err(e) => {
                println!("create_window error: {}", e);
            }
        }
    }
    println!("寄，没拿到 app handle");
    // let new_window = tauri::window::WindowBuilder::new(
    //     app_handle,
    //     label.to_string(),
    //     tauri::WindowUrl::App(url.into()),
    // )
    // .title(title)
    // .center()
    // .visible(false)
    // .resizable(window_info.resizable)
    // .fullscreen(window_info.fullscreenable)
    // .always_on_top(window_info.always_on_top)
    // .inner_size(window_info.width, window_info.height)
    // .transparent(window_info.transparent)
    // .decorations(window_info.decorations)
    // .skip_taskbar(window_info.skip_taskbar)
    // .center()
    // .build();
    // match new_window {
    //     Ok(window) => {
    //         println!("new window");
    //         let _ = window.show();
    //         let _ = window.set_focus();
    //         if let WindowType::Main = window_type {
    //             set_shadow(&window, true).expect("Unsupported platform!");
    //         }
    //     }
    //     Err(e) => {
    //         println!("create_window error: {}", e);
    //     }
    // }
    Ok(())
}

// #[tauri::command]
// pub fn write_to_clip(id: u64) -> bool {
//     let record = SqliteDB::new().find_by_id(id);
//     match record {
//         Ok(r) => {
//             if r.data_type == "text" {
//                 let _ = ClipBoardOprator::set_text(r.content);
//             } else if r.data_type == "image" {
//                 let image_data: ImageDataDB = json_util::parse(&r.content).unwrap();
//                 let _ = ClipBoardOprator::set_image(image_data);
//             }
//             true
//         }
//         Err(e) => {
//             println!("err:{}", e);
//             false
//         }
//     }
// }

// #[tauri::command]
// pub fn focus_previous_window() -> CmdResult {
//     focus_window(*PreviousProcessId.lock().unwrap());
//     Ok(())
// }

// #[tauri::command]
// pub fn paste_in_previous_window() -> CmdResult {
//     focus_window(*PreviousProcessId.lock().unwrap());
//     dispatch_util::paste();
//     Ok(())
// }
