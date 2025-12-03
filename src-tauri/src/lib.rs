use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tauri::{
    AppHandle,
    Emitter,
    Manager,
    Url,
    WebviewUrl,
    WebviewWindow,
    WebviewWindowBuilder,
    Window,
    WindowEvent,
};

const LOGIN_WINDOW_LABEL: &str = "login-window";
const LOGIN_URL: &str = "https://zu.zuhaowan.com";
const LOGIN_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36";
const AUTH_SNIFFER_SCRIPT: &str = include_str!("scripts/auth_sniffer.js");

#[derive(Serialize, Clone)]
struct LoginPayload {
    token: Option<String>,
    cookie: Option<String>,
    source: String,
    captured_at: u64,
}

#[derive(Serialize, Clone)]
struct LoginWindowStatus {
    phase: String,
    message: String,
    timestamp: u64,
}

fn now_millis() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis()
        .try_into()
        .map_err(|_| "timestamp overflow".to_string())
}

fn emit_login_status<M: Into<String>>(app: &AppHandle, phase: &str, message: M) {
    if let Ok(timestamp) = now_millis() {
        let payload = LoginWindowStatus {
            phase: phase.to_string(),
            message: message.into(),
            timestamp,
        };
        let _ = app.emit("login-window-status", &payload);
    }
}

fn login_window(handle: &AppHandle) -> Option<WebviewWindow> {
    handle.get_webview_window(LOGIN_WINDOW_LABEL)
}

#[tauri::command]
fn open_login_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = login_window(&app) {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        let _ = window.eval(AUTH_SNIFFER_SCRIPT);
        emit_login_status(&app, "reuse", "登录窗口已重新唤起");
        return Ok(());
    }

    let url = Url::parse(LOGIN_URL).map_err(|e| e.to_string())?;
    let mut builder = WebviewWindowBuilder::new(&app, LOGIN_WINDOW_LABEL, WebviewUrl::External(url))
        .title("租号玩登录")
        .visible(true)
        .resizable(true)
        .closable(true)
        .decorations(true)
        .center()
        .transparent(false)
        .min_inner_size(360.0, 560.0)
        .inner_size(420.0, 720.0)
        .user_agent(LOGIN_USER_AGENT)
        .initialization_script(AUTH_SNIFFER_SCRIPT);

    #[cfg(debug_assertions)]
    {
        builder = builder.devtools(true);
    }

    builder.build().map_err(|e| e.to_string())?;

    if let Some(window) = login_window(&app) {
        let _ = window.set_focus();
    }

    emit_login_status(&app, "created", "登录窗口已创建，正在加载页面…");

    Ok(())
}

#[tauri::command]
fn close_login_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = login_window(&app) {
        window.close().map_err(|e| e.to_string())?;
        emit_login_status(&app, "closed", "已尝试关闭登录窗口");
    }
    Ok(())
}

#[tauri::command]
fn reload_login_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = login_window(&app) {
        window
            .eval("window.location.reload();")
            .map_err(|e| e.to_string())?;
        emit_login_status(&app, "reload", "刷新指令已发送");
        Ok(())
    } else {
        emit_login_status(&app, "reload", "窗口尚未创建，正在重新打开…");
        open_login_window(app)
    }
}

#[tauri::command]
fn focus_login_window(app: AppHandle) -> Result<(), String> {
    let window = login_window(&app).ok_or_else(|| "登录窗口尚未创建".to_string())?;
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    emit_login_status(&app, "focus", "已尝试将登录窗口置于前台");
    Ok(())
}

#[tauri::command]
fn open_login_devtools(app: AppHandle) -> Result<(), String> {
    let window = login_window(&app).ok_or_else(|| "登录窗口尚未创建".to_string())?;
    window.open_devtools();
    emit_login_status(&app, "devtools", "调试面板已请求打开 (Ctrl+Shift+I)");
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
        captured_at: now_millis()?,
    };

    app.emit("login-info", &payload)
        .map_err(|e| e.to_string())?;

    emit_login_status(&app, "captured", "已捕获登录 token / cookie");

    if window.label() == LOGIN_WINDOW_LABEL {
        let _ = window.close();
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            open_login_window,
            close_login_window,
            reload_login_window,
            focus_login_window,
            open_login_devtools,
            report_login_state
        ])
        .on_page_load(|window, _| {
            if window.label() == LOGIN_WINDOW_LABEL {
                let app = window.app_handle();
                emit_login_status(&app, "page-ready", "租号玩页面加载完成，已注入捕获脚本");
                let _ = window.eval(AUTH_SNIFFER_SCRIPT);
            }
        })
        .on_window_event(|window, event| {
            if window.label() == LOGIN_WINDOW_LABEL {
                let app = window.app_handle();
                match event {
                    WindowEvent::CloseRequested { .. } => {
                        emit_login_status(&app, "close-requested", "用户尝试关闭登录窗口");
                    }
                    WindowEvent::Destroyed => {
                        emit_login_status(&app, "closed", "登录窗口已关闭");
                    }
                    _ => {}
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
