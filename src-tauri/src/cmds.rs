use chrono::Local;
use parking_lot::MutexGuard;
use tauri::{AppHandle, Manager, State, Window};

use crate::{
    core::{
        clipboard::{self, ClipBoardOprator},
        database::{ImageDataDB, QueryReq, Record, SqliteDB},
        global::GLOBAL,
    },
    utils::{
        dispatch_util::{self, sleep},
        json_util,
        visible::is_window_visible,
        window_util::{focus_window, get_active_process_id},
    },
    GAppHandle, PreviousProcessId,
};

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub async fn get_clipboard_data() -> CmdResult<String> {
    clipboard::get_clipboard_data().await
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
pub fn print(msg: String) -> CmdResult {
    println!("cmd-p:\n\t{}", msg);
    Ok(())
}

#[tauri::command]
pub fn escape_win() -> CmdResult {
    println!("[{}] in escape_win", Local::now());
    {
        let binding = GLOBAL.lock();
        let (opt_win, _) = binding.get_window();
        if let Some(window) = opt_win {
            window.close().unwrap();
        }
    }
    _ = focus_previous_window();
    println!("[{}] out escape_win", Local::now());
    Ok(())
}

#[tauri::command]
pub fn open_window() -> CmdResult {
    println!("[{}] in open_window", Local::now());

    // GLOBAL.lock()
    let mut opt_win: Option<Window> = None;
    let mut is_new = false;
    {
        let mut binding = GLOBAL.lock();
        binding.set_pre_process_id(get_active_process_id());
        (opt_win, is_new) = binding.get_window();
    }
    // println!("{}", is_new);
    if let Some(window) = opt_win {
        if !is_new {
            if window.is_visible().unwrap() {
                let _ = window.close();
                println!("[{}] out open_window", Local::now());
                return Ok(());
            }
            let _ = window.unminimize();
            _ = window.show();
            _ = window.set_focus();
            println!("[{}] out open_window", Local::now());
            return Ok(());
        } else {
            let _ = window.show();
            _ = window.set_focus();
        }
    } else {
        println!("寄，没拿到 app handle");
    }
    println!("[{}] out open_window", Local::now());
    Ok(())
}

#[tauri::command]
pub fn write_to_clip(id: u64) -> bool {
    println!("[{}] in write_to_clip", Local::now());

    let record = SqliteDB::new().find_by_id(id);
    match record {
        Ok(r) => {
            println!("id:{} record:{:?}", id, r.content);
            if r.data_type == "text" {
                let _ = ClipBoardOprator::set_text(r.content);
            } else if r.data_type == "image" {
                let image_data: ImageDataDB = json_util::parse(&r.content).unwrap();
                let _ = ClipBoardOprator::set_image(image_data);
            }

            // sleep(2000);

            println!("[{}] out write_to_clip", Local::now());
            true
        }
        Err(e) => {
            println!("write_to_clip Err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn focus_previous_window() -> CmdResult {
    let mut pre_process_id = 0;
    {
        pre_process_id = GLOBAL.lock().get_pre_process_id();
    }
    focus_window(pre_process_id);
    // focus_window(*PreviousProcessId.lock().unwrap());
    Ok(())
}

#[tauri::command]
pub fn paste_in_previous_window() -> CmdResult {
    println!("[{}] in paste_in_previous_window", Local::now());

    let mut pre_process_id;
    {
        pre_process_id = GLOBAL.lock().get_pre_process_id();
    }
    println!("get pre process id:{}", pre_process_id);

    if pre_process_id == 0 {
        println!("[{}] out paste_in_previous_window", Local::now());
        return Ok(());
    }

    focus_window(pre_process_id);
    sleep(100);
    dispatch_util::paste();

    println!("[{}] out paste_in_previous_window", Local::now());

    Ok(())
}
