#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager, WindowEvent};

struct AppState {
    login_window_label: Arc<Mutex<Option<String>>>,
}

#[tauri::command]
async fn open_login_window(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut window_label = state.login_window_label.lock().unwrap();

    let label = "login-window";

    if let Some(existing_label) = window_label.as_ref() {
        if let Some(existing_window) = app.get_webview_window(existing_label) {
            existing_window.set_focus().map_err(|e| e.to_string())?;
            return Ok(());
        }
    }

    *window_label = Some(label.to_string());

    let login_window = tauri::webview::WebviewWindowBuilder::new(
        &app,
        label,
        tauri::WebviewUrl::External("https://zu.zuhaowan.com".parse().unwrap()),
    )
    .title("Login - Zuhaowan")
    .inner_size(600.0, 700.0)
    .resizable(true)
    .center()
    .decorations(true)
    .transparent(false)
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
    .build()
    .map_err(|e| e.to_string())?;

    app.emit("login-window-opened", ()).ok();

    let label_mutex = state.login_window_label.clone();
    let app_handle = app.clone();

    login_window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { .. } = event {
            let mut window_label = label_mutex.lock().unwrap();
            *window_label = None;
            app_handle.emit("login-window-closed", ()).ok();
        }
    });

    Ok(())
}

#[tauri::command]
async fn close_login_window(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut window_label = state.login_window_label.lock().unwrap();

    if let Some(label) = window_label.as_ref() {
        if let Some(window) = app.get_webview_window(label) {
            window.close().map_err(|e| e.to_string())?;
        }
        *window_label = None;
        app.emit("login-window-closed", ()).ok();
    }

    Ok(())
}

#[tauri::command]
async fn is_login_window_open(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    let window_label = state.login_window_label.lock().unwrap();

    if let Some(label) = window_label.as_ref() {
        Ok(app.get_webview_window(label).is_some())
    } else {
        Ok(false)
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            login_window_label: Arc::new(Mutex::new(None)),
        })
        .setup(|_app| {
            println!("Application initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_login_window,
            close_login_window,
            is_login_window_open,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
