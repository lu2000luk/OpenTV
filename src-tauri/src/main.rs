// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command// Add `monitor` to the import list

use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager, Url, WebviewUrl};
use tauri::webview::WebviewBuilder;

const INIT_SCRIPT: &str = r#"

"#;


fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            
            let monitor = window.current_monitor();

            let _size = match monitor {
                Ok(Some(monitor)) => {
                    let size = monitor.size();
                    Some((size.width, size.height))
                },
                Ok(None) => None,
                Err(_) => todo!(),
            };

            let (_width, _height) = _size.unwrap_or((800, 600));

            let webview_width = _width / 5 * 4;

            let _webview1 = window.add_child(
                tauri::webview::WebviewBuilder::new("main1", WebviewUrl::App("https://google.com/".into()))
                    .initialization_script(INIT_SCRIPT)
                    .auto_resize(),
                LogicalPosition::new(_width / 5, 0),
                LogicalSize::new(webview_width, _height),
            )?;

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_cors_fetch::init())
        .invoke_handler(tauri::generate_handler![get_size, resize_webview, reposition_webview, navigate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn resize_webview(app: AppHandle, _size: LogicalSize<u32>) {
    let main_webview = app.get_webview("main1").unwrap();

    main_webview.set_size(_size).unwrap();
}

#[tauri::command]
fn reposition_webview(app: AppHandle, _pos: LogicalPosition<u32>) {
    let main_webview = app.get_webview("main1").unwrap();

    main_webview.set_position(_pos).unwrap();
}

#[tauri::command]
fn navigate(app: AppHandle, _url: Url) {
    let mut main_webview = app.get_webview("main1").unwrap();

    main_webview.navigate(_url);
}

#[tauri::command]
fn get_size(app: AppHandle) -> LogicalSize<u32> {
    let window = app.get_window("main").unwrap();
    let monitor = window.current_monitor().unwrap().unwrap();
    let size = monitor.size();

    LogicalSize::new(size.width, size.height).into()
}