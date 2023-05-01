use super::database::{self, ImageDataDB};
use crate::core::database::Record;
use crate::utils::{img_util, json_util, string_util};
use arboard::Clipboard;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use chrono::Duration;
use serde_json::json;
use std::thread;
use anyhow::Result;



pub async fn get_clipboard_data() -> Result<String, String> {
    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    let mut clipboard = Clipboard::new().unwrap();
    let text: Option<_> = match clipboard.get_text() {
        Ok(text) => Some(text),
        Err(_) => None,
    };

    let img: Option<_> = match clipboard.get_image() {
        Ok(image) => {
            // let image_data = image.;
            let base64_image: String = CUSTOM_ENGINE.encode(&image.bytes);
            Some(base64_image)
        }
        Err(_) => None,
    };

    let result = json!({
      "text": text,
      "image": img
    })
    .to_string();

    Ok(result)
}
pub struct ClipBoardOprator;

impl ClipBoardOprator {
    pub fn set_text(text: String) -> Result<()> {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(text)?;
        Ok(())
    }

    pub fn set_image(data: ImageDataDB) -> Result<()> {
        let mut clipboard = Clipboard::new()?;
        let img_data = img_util::base64_to_rgba8(&data.base64).unwrap();
        clipboard.set_image(img_data)?;
        Ok(())
    }
}

pub struct ClipboardWatcher;

impl ClipboardWatcher {
    pub fn start() {
        tauri::async_runtime::spawn(async {
            // 1000毫秒检测一次剪切板变化
            let wait_millis = 1000i64;
            let mut last_content_md5 = String::new();
            let mut last_img_md5 = String::new();
            let mut clipboard = Clipboard::new().unwrap();
            println!("start clipboard watcher");
            let mut cnt = 1;
            loop {

                // println!("before db new");
                let db = database::SqliteDB::new();
                // println!("after db new");
                let text = clipboard.get_text();
                
                // println!("!!!!!!!!! cnt: {} text: {:?}", cnt, text);
                cnt = cnt + 1;

                let _ = text.map(|text| {
                    let content_origin = text.clone();
                    let content = text.trim();
                    let md5 = string_util::md5(&content_origin);
                    if !content.is_empty() && md5 != last_content_md5 {
                        println!("===== new content: {}", content);
                        // 说明有新内容
                        let content_preview = if content.len() > 1000 {
                            Some(content.chars().take(1000).collect())
                        } else {
                            Some(content.to_string())
                        };
                        let res = db.insert_if_not_exist(Record {
                            content: content_origin,
                            content_preview,
                            data_type: "text".to_string(),
                            pined: false,
                            ..Default::default()
                        });
                        match res {
                            Ok(_) => {}
                            Err(e) => {
                                println!("insert record error: {}", e);
                            }
                        }
                        last_content_md5 = md5;
                    }
                });

                let img = clipboard.get_image();
                let _ = img.map(|img| {
                    let img_md5 = string_util::md5_by_bytes(&img.bytes);
                    if img_md5 != last_img_md5 {
                        // 有新图片产生
                        let base64 = img_util::rgba8_to_base64(&img);
                        let content_db = ImageDataDB {
                            width: img.width,
                            height: img.height,
                            base64,
                        };
                        // 压缩画质作为预览图，防止渲染时非常卡顿
                        let jpeg_base64 = img_util::rgba8_to_jpeg_base64(&img, 75);
                        let content_preview_db = ImageDataDB {
                            width: img.width,
                            height: img.height,
                            base64: jpeg_base64,
                        };
                        let content = json_util::stringfy(&content_db).unwrap();
                        let content_preview = json_util::stringfy(&content_preview_db).unwrap();
                        let res = db.insert_if_not_exist(Record {
                            content,
                            content_preview: Some(content_preview),
                            data_type: "image".to_string(),
                            pined: false,
                            ..Default::default()
                        });
                        match res {
                            Ok(_) => {}
                            Err(e) => {
                                println!("insert record error: {}", e);
                            }
                        }
                        last_img_md5 = img_md5;
                    }
                });
                // let limit = Config::common().latest().record_limit.clone();
                let limit = Option::Some(10);
                if let Some(l) = limit {
                    let _ = db.delete_over_limit(l as usize);
                }
                thread::sleep(Duration::milliseconds(wait_millis).to_std().unwrap());
            }
        });
    }
}
