use arboard::Clipboard;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use serde_json::json;
use tauri::Window;

#[tauri::command]
pub async fn get_clipboard_data(window: Window) -> Result<String, String> {
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
