// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command// Add `monitor` to the import list

use tauri::{LogicalPosition, LogicalSize, Manager, WebviewUrl};

fn main() {
    tauri::Builder::default().setup(|app| {
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
                    .auto_resize(),
                LogicalPosition::new(_width / 5, 0),
                LogicalSize::new(webview_width, _height),
            )?;

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_cors_fetch::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
