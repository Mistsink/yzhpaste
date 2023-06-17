use crate::{
    config,
    config::{CfgHotkeys, CommonConfig, Config},
    core::{
        clipboard::{self, ClipBoardOprator},
        database::{ImageDataDB, QueryReq, Record, SqliteDB},
        global::GLOBAL,
    },
    log_err,
    utils::{
        dispatch_util, json_util,
        window_util::{focus_window, get_active_process_info, ProcessInfo},
    },
};
use chrono::Local;

type CmdResult<T = ()> = Result<T, String>;

// config 相关
#[tauri::command]
pub fn get_common_config() -> CmdResult<CommonConfig> {
    Ok(Config::common().data().clone())
}

#[tauri::command]
pub fn set_common_config(config: CommonConfig) -> CmdResult {
    Config::common().draft().patch_config(config);
    Config::common().apply();
    log_err!(Config::common().data().save_file());

    // todo enable_auto_launch
    // todo hotkeys
    Ok(())
}

#[tauri::command]
pub async fn change_language(language: String) -> CmdResult {
    let _ = config::modify_common_config(CommonConfig {
        language: Some(language),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}

#[tauri::command]
pub async fn change_record_limit(limit: u32) -> CmdResult {
    let _ = config::modify_common_config(CommonConfig {
        record_limit: Some(limit),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}

#[tauri::command]
pub async fn change_auto_launch(enable: bool) -> CmdResult {
    let _ = config::modify_common_config(CommonConfig {
        enable_auto_launch: Some(enable),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}

#[tauri::command]
pub async fn change_auto_paste(enable: bool) -> CmdResult {
    if enable == true {
        dispatch_util::request_permissions();
    }
    let _ = config::modify_common_config(CommonConfig {
        enable_auto_paste: Some(enable),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}

#[tauri::command]
pub async fn change_delete_confirm(enable: bool) -> CmdResult {
    let _ = config::modify_common_config(CommonConfig {
        enable_delete_confirm: Some(enable),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}

#[tauri::command]
pub async fn change_theme_mode(theme_mode: String) -> CmdResult {
    let _ = config::modify_common_config(CommonConfig {
        theme_mode: Some(theme_mode),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}

#[tauri::command]
pub async fn change_hotkeys(hotkeys: CfgHotkeys) -> CmdResult {
    let _ = config::modify_common_config(CommonConfig {
        hotkeys: Some(hotkeys),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}

//  record 相关
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
            // slide_up(&window);
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
    // let mut opt_win: Option<Window> = None;
    // let mut is_new = false;
    // {
    let mut binding = GLOBAL.lock();
    binding.set_pre_process_info(get_active_process_info());

    let (opt_win, is_new) = binding.get_window();
    // }
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
    // let mut pre_process_id = 0;
    // {
    let pre_process_info = GLOBAL.lock().get_pre_process_info();
    // }
    focus_window(&pre_process_info);
    Ok(())
}

#[tauri::command]
pub fn paste_in_previous_window() -> CmdResult {
    println!("[{}] in paste_in_previous_window", Local::now());

    // let mut pre_process_id;
    // {
    let pre_process_info = GLOBAL.lock().get_pre_process_info();
    // }
    println!("get pre process id:{}", pre_process_info.process_id);

    if pre_process_info.process_id == 0 {
        println!("[{}] out paste_in_previous_window", Local::now());
        return Ok(());
    }

    focus_window(&pre_process_info);
    dispatch_util::paste();

    println!("[{}] out paste_in_previous_window", Local::now());

    Ok(())
}
