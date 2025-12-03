use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tauri::{AppHandle, Manager, Url, WebviewUrl, WebviewWindowBuilder, Window};

const LOGIN_WINDOW_LABEL: &str = "login-window";
const LOGIN_URL: &str = "https://zu.zuhaowan.com";
const AUTH_SNIFFER_SCRIPT: &str = include_str!("scripts/auth_sniffer.js");

#[derive(Serialize, Clone)]
struct LoginPayload {
    token: Option<String>,
    cookie: Option<String>,
    source: String,
    captured_at: u128,
}

#[tauri::command]
fn open_login_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(LOGIN_WINDOW_LABEL) {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        let _ = window.eval(AUTH_SNIFFER_SCRIPT);
        return Ok(());
    }

    let url = Url::parse(LOGIN_URL).map_err(|e| e.to_string())?;
    WebviewWindowBuilder::new(&app, LOGIN_WINDOW_LABEL, WebviewUrl::External(url))
        .title("租号玩登录")
        .inner_size(420.0, 720.0)
        .min_inner_size(360.0, 560.0)
        .resizable(true)
        .center()
        .initialization_script(AUTH_SNIFFER_SCRIPT)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn report_login_state(
    app: AppHandle,
    window: Window,
    token: Option<String>,
    cookie: Option<String>,
) -> Result<(), String> {
    let normalized_token = token.and_then(|value| {
        let trimmed = value.trim().to_string();
        (!trimmed.is_empty()).then_some(trimmed)
    });

    let normalized_cookie = cookie.and_then(|value| {
        let trimmed = value.trim().to_string();
        (!trimmed.is_empty()).then_some(trimmed)
    });

    if normalized_token.is_none() && normalized_cookie.is_none() {
        return Ok(());
    }

    let payload = LoginPayload {
        token: normalized_token,
        cookie: normalized_cookie,
        source: window.label().to_string(),
        captured_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_millis(),
    };

    app.emit_all("login-info", &payload)
        .map_err(|e| e.to_string())?;

    if window.label() == LOGIN_WINDOW_LABEL {
        let _ = window.close();
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_login_window, report_login_state])
        .on_page_load(|window, _| {
            if window.label() == LOGIN_WINDOW_LABEL {
                let _ = window.eval(AUTH_SNIFFER_SCRIPT);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
