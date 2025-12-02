// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    token: String,
    user: String,
}

#[tauri::command]
async fn open_login_window(app: tauri::AppHandle) -> Result<(), String> {
    let window = tauri::WebviewWindowBuilder::new(
        &app,
        "login",
        tauri::WebviewUrl::External("https://login.example.com".parse().unwrap())
    )
    .title("Login")
    .inner_size(800.0, 600.0)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn close_login_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("login") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn store_auth_token(app: tauri::AppHandle, token: String) -> Result<(), String> {
    // Store authentication token securely using tauri-plugin-store
    // This is a placeholder - actual implementation would use the store plugin
    println!("Storing auth token securely");
    Ok(())
}

#[tauri::command]
async fn fetch_from_api(endpoint: String) -> Result<String, String> {
    // Use tauri-plugin-http to make requests that bypass CORS
    let client = tauri_plugin_http::reqwest::Client::new();
    
    let response = client
        .get(&format!("https://api.example.com/{}", endpoint))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    Ok(body)
}

fn main() {
    tauri::Builder::default()
        // Initialize plugins with secure configuration
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_autostart::init(
                tauri_plugin_autostart::MacosLauncher::LaunchAgent,
                Some(vec![]),
            )
        )
        .invoke_handler(tauri::generate_handler![
            open_login_window,
            close_login_window,
            store_auth_token,
            fetch_from_api,
        ])
        .setup(|app| {
            // Setup logic here
            println!("Application started with secure cross-origin configuration");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
