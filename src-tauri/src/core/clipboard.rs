use super::database::{self, ImageDataDB};
use crate::config::Config;
use crate::core::database::Record;
use crate::events::on_records_update;
use crate::utils::{img_util, json_util, string_util};
use anyhow::Result;
use arboard::Clipboard;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use chrono::Duration;
use serde_json::json;
use tauri::async_runtime::JoinHandle;
use std::sync::{Arc, Mutex};
use std::thread;

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

#[derive(Debug, Default, Clone)]
pub struct ClipboardWatcher {
    stop_flag: Arc<Mutex<bool>>,
    running: Arc<Mutex<bool>>,
    thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl ClipboardWatcher {
    pub fn new() -> Self {
        ClipboardWatcher {
            stop_flag: Arc::new(Mutex::new(false)),
            running: Arc::new(Mutex::new(false)),
            thread_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self) {
        {
            let mut running = self.running.lock().unwrap();
            if *running {
                println!("Already running, please stop before starting again.");
                return;
            }
            *running = true;
        }

        let stop_flag = self.stop_flag.clone();
        let running = self.running.clone();
        let thread_handle = self.thread_handle.clone();
        let handle = tauri::async_runtime::spawn(async move {
            // 1000毫秒检测一次剪切板变化
            let wait_millis = 1000i64;
            let mut last_content_md5 = String::new();
            let mut last_img_md5 = String::new();
            let mut last_md5 = String::new();
            let mut clipboard = Clipboard::new().unwrap();
            println!("start clipboard watcher");
            let mut cnt = 1;

            let mut watch_fn = || {
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

                    if !content.is_empty() && md5 != last_md5 {
                        // if !content.is_empty() && md5 != last_content_md5 {
                        println!("===== new content: {}", content);
                        // 说明有新内容
                        let limit_len = 999999;
                        let content_preview = if content.len() > limit_len {
                            Some(content.chars().take(limit_len).collect())
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
                        // last_content_md5 = md5;

                        on_records_update();
                        last_md5 = md5;
                    }
                });

                let img = clipboard.get_image();
                let _ = img.map(|img| {
                    let img_md5 = string_util::md5_by_bytes(&img.bytes);
                    if img_md5 != last_md5 {
                        // if img_md5 != last_img_md5 {
                        println!("===== new image: {} x {}", img.width, img.height);
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
                        // last_img_md5 = img_md5;
                        on_records_update();
                        last_md5 = img_md5;
                    }
                });
                let limit = Config::common().latest().record_limit.clone();
                if let Some(l) = limit {
                    let _ = db.delete_over_limit(l as usize);
                }
                thread::sleep(Duration::milliseconds(wait_millis).to_std().unwrap());
            };
            loop {
                let should_stop = {
                    let lock = stop_flag.lock().unwrap();
                    *lock
                };

                if should_stop {
                    let mut running = running.lock().unwrap();
                    *running = false;
                    break;
                }
                watch_fn();
            }
        });
        *thread_handle.lock().unwrap() = Some(handle);
    }

    pub fn stop(&self) {
        {
            let mut stop_flag = self.stop_flag.lock().unwrap();
            *stop_flag = true;
        }
        if let Some(handle) = self.thread_handle.lock().unwrap().take() {
            // handle.join().unwrap();
            handle.abort();
        }
    }
}
