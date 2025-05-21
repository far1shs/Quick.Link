mod script;

use std::path::PathBuf;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use script::download::download_file;
use script::frp::{run_frp, stop_frp, get_frp_logs};
#[tauri::command]
fn check_file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[tauri::command]
fn get_current_dir() -> Result<String, String> {
    match std::env::current_dir() {
        Ok(path) => path.to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "路径包含无效 UTF-8".to_string()),
        Err(e) => Err(format!("无法获取当前目录: {}", e)),
    }
}

#[tauri::command]
fn join_paths(base: String, sub: String) -> Result<String, String> {
    let mut path = PathBuf::from(base);
    path.push(sub);
    path.to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "路径包含非法字符".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            script::tray::init(app_handle.clone())?;
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            run_frp,
            stop_frp,
            get_frp_logs,
            download_file,
            check_file_exists,
            get_current_dir,
            join_paths
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
