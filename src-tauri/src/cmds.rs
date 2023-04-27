use crate::core::{
    clipboard_listener,
    database::{QueryReq, Record, SqliteDB},
};

#[tauri::command]
pub async fn get_clipboard_data() -> Result<String, String> {
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
pub fn find_all_record() -> Vec<Record> {
    SqliteDB::new().find_all().unwrap()
}

#[tauri::command]
pub fn mark_favorite(id: u64) -> bool {
    match SqliteDB::new().mark_favorite(id) {
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
