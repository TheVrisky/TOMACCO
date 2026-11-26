//! TOMACCO browser core.
//!
//! Phase 0+1a: minimal Tauri 2 application skeleton.
//! Browser Core (tabs, navigation, WebView Controller) is implemented in Phase 1.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running TOMACCO");
}
