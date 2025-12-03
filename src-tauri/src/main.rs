// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginData {
    token: String,
    cookies: String,
    url: String,
    timestamp: String,
}

#[tauri::command]
async fn open_login_window(app: tauri::AppHandle) -> Result<(), String> {
    println!("正在创建登录窗口...");
    
    let login_window = WebviewWindowBuilder::new(
        &app,
        "login",
        WebviewUrl::External("https://zu.zuhaowan.com".parse().unwrap())
    )
    .title("登录 - 租好玩")
    .inner_size(800.0, 600.0)
    .center()
    .resizable(true)
    .build()
    .map_err(|e| format!("创建窗口失败: {}", e))?;

    println!("登录窗口已创建");

    let window_for_inject = login_window.clone();
    
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
        
        let inject_script = r#"
            (function() {
                console.log('Tauri login capture script loading...');
                
                window.__TAURI_LOGIN_CAPTURE__ = {
                    token: '',
                    cookies: '',
                    url: '',
                    allData: {},
                    timestamp: '',
                    updated: false
                };
                
                function captureAndStore() {
                    try {
                        let token = '';
                        let allData = {};
                        
                        if (typeof localStorage !== 'undefined') {
                            for (let i = 0; i < localStorage.length; i++) {
                                let key = localStorage.key(i);
                                let value = localStorage.getItem(key);
                                allData['localStorage_' + key] = value;
                                
                                if (key.toLowerCase().includes('token') || 
                                    key.toLowerCase().includes('auth') ||
                                    key.toLowerCase().includes('session') ||
                                    key.toLowerCase().includes('user')) {
                                    if (!token) token = value;
                                }
                            }
                        }
                        
                        if (typeof sessionStorage !== 'undefined') {
                            for (let i = 0; i < sessionStorage.length; i++) {
                                let key = sessionStorage.key(i);
                                let value = sessionStorage.getItem(key);
                                allData['sessionStorage_' + key] = value;
                                
                                if (key.toLowerCase().includes('token') || 
                                    key.toLowerCase().includes('auth') ||
                                    key.toLowerCase().includes('session') ||
                                    key.toLowerCase().includes('user')) {
                                    if (!token) token = value;
                                }
                            }
                        }
                        
                        let cookies = document.cookie;
                        
                        if (cookies) {
                            let cookiePairs = cookies.split(';');
                            for (let pair of cookiePairs) {
                                let parts = pair.trim().split('=');
                                if (parts.length >= 2) {
                                    let key = parts[0];
                                    let value = parts.slice(1).join('=');
                                    allData['cookie_' + key] = value;
                                    
                                    if (key.toLowerCase().includes('token') || 
                                        key.toLowerCase().includes('auth') ||
                                        key.toLowerCase().includes('session') ||
                                        key.toLowerCase().includes('user')) {
                                        if (!token) token = value;
                                    }
                                }
                            }
                        }
                        
                        if (token || cookies) {
                            window.__TAURI_LOGIN_CAPTURE__ = {
                                token: token,
                                cookies: cookies,
                                url: window.location.href,
                                allData: allData,
                                timestamp: new Date().toISOString(),
                                updated: true
                            };
                            console.log('Captured login data:', window.__TAURI_LOGIN_CAPTURE__);
                        }
                    } catch (e) {
                        console.error('Error in captureAndStore:', e);
                    }
                }
                
                const originalSetItem = localStorage.setItem;
                localStorage.setItem = function(key, value) {
                    console.log('localStorage.setItem:', key, value);
                    originalSetItem.apply(this, arguments);
                    
                    if (key.toLowerCase().includes('token') || 
                        key.toLowerCase().includes('auth') ||
                        key.toLowerCase().includes('session') ||
                        key.toLowerCase().includes('user')) {
                        captureAndStore();
                    }
                };
                
                const originalSessionSetItem = sessionStorage.setItem;
                sessionStorage.setItem = function(key, value) {
                    console.log('sessionStorage.setItem:', key, value);
                    originalSessionSetItem.apply(this, arguments);
                    
                    if (key.toLowerCase().includes('token') || 
                        key.toLowerCase().includes('auth') ||
                        key.toLowerCase().includes('session') ||
                        key.toLowerCase().includes('user')) {
                        captureAndStore();
                    }
                };
                
                setInterval(captureAndStore, 2000);
                
                console.log('Tauri login capture script injected successfully');
            })();
        "#;
        
        if let Err(e) = window_for_inject.eval(inject_script) {
            println!("注入脚本失败: {}", e);
        } else {
            println!("已注入捕获脚本");
        }
    });

    let window_for_timer = login_window.clone();
    let app_for_timer = app.clone();
    
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
        let mut attempts = 0;
        let max_attempts = 120;
        let mut last_token = String::new();
        
        loop {
            interval.tick().await;
            attempts += 1;
            
            if attempts > max_attempts {
                println!("停止捕获检查（超时）");
                break;
            }
            
            if window_for_timer.is_closable().is_err() {
                println!("登录窗口已关闭");
                break;
            }
            
            let read_script = r#"
                (function() {
                    try {
                        if (window.__TAURI_LOGIN_CAPTURE__ && window.__TAURI_LOGIN_CAPTURE__.updated) {
                            let data = window.__TAURI_LOGIN_CAPTURE__;
                            let result = JSON.stringify({
                                token: data.token || '',
                                cookies: data.cookies || '',
                                url: data.url || window.location.href,
                                allData: JSON.stringify(data.allData || {}),
                                timestamp: data.timestamp || new Date().toISOString()
                            });
                            let metaElement = document.createElement('meta');
                            metaElement.name = '__tauri_captured_data__';
                            metaElement.content = result;
                            let oldMeta = document.querySelector('meta[name="__tauri_captured_data__"]');
                            if (oldMeta) {
                                oldMeta.remove();
                            }
                            document.head.appendChild(metaElement);
                        }
                    } catch (e) {
                        console.error('Error storing data in meta:', e);
                    }
                })();
            "#;
            
            if let Err(e) = window_for_timer.eval(read_script) {
                println!("注入读取脚本失败: {}", e);
                continue;
            }
            
            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
            
            let extract_script = r#"
                (function() {
                    let metaElement = document.querySelector('meta[name="__tauri_captured_data__"]');
                    if (metaElement) {
                        document.title = 'DATA:' + metaElement.content;
                    } else {
                        document.title = 'NO_DATA';
                    }
                })();
            "#;
            
            if let Err(e) = window_for_timer.eval(extract_script) {
                println!("提取数据失败: {}", e);
                continue;
            }
            
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            
            match window_for_timer.title() {
                Ok(title) => {
                    if title.starts_with("DATA:") {
                        let data_str = &title[5..];
                        
                        match serde_json::from_str::<serde_json::Value>(data_str) {
                            Ok(data) => {
                                if let (Some(token), Some(cookies), Some(url), Some(timestamp)) = (
                                    data.get("token").and_then(|v| v.as_str()),
                                    data.get("cookies").and_then(|v| v.as_str()),
                                    data.get("url").and_then(|v| v.as_str()),
                                    data.get("timestamp").and_then(|v| v.as_str())
                                ) {
                                    if !token.is_empty() && token != last_token {
                                        println!("========== 捕获到登录数据 ==========");
                                        println!("Token: {}", token);
                                        println!("Cookies: {}", cookies);
                                        println!("URL: {}", url);
                                        println!("时间戳: {}", timestamp);
                                        
                                        let all_data_str = data.get("allData")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("{}");
                                        println!("全部数据: {}", all_data_str);
                                        println!("===================================");
                                        
                                        let login_data = serde_json::json!({
                                            "token": token,
                                            "cookies": cookies,
                                            "url": url,
                                            "allData": all_data_str,
                                            "timestamp": timestamp
                                        });
                                        
                                        if let Some(main_window) = app_for_timer.get_webview_window("main") {
                                            match main_window.emit("login-data-captured", login_data) {
                                                Ok(_) => {
                                                    println!("已发送数据到主窗口");
                                                    last_token = token.to_string();
                                                }
                                                Err(e) => {
                                                    println!("发送事件失败: {}", e);
                                                }
                                            }
                                        }
                                        
                                        let reset_title = r#"
                                            document.title = '登录 - 租好玩';
                                        "#;
                                        let _ = window_for_timer.eval(reset_title);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("解析数据失败: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("获取标题失败: {}", e);
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn capture_login_data(
    token: String,
    cookies: String,
    url: String,
    all_data: Option<String>,
    source: Option<String>,
    timestamp: String,
    app: tauri::AppHandle
) -> Result<(), String> {
    println!("========== 收到手动捕获的登录数据 ==========");
    println!("Token: {}", token);
    println!("Cookies: {}", cookies);
    println!("URL: {}", url);
    if let Some(src) = &source {
        println!("来源: {}", src);
    }
    if let Some(data) = &all_data {
        println!("全部数据: {}", data);
    }
    println!("时间戳: {}", timestamp);
    println!("=========================================");
    
    let login_data = serde_json::json!({
        "token": token,
        "cookies": cookies,
        "url": url,
        "allData": all_data.unwrap_or_default(),
        "source": source.unwrap_or_default(),
        "timestamp": timestamp
    });
    
    if let Some(main_window) = app.get_webview_window("main") {
        main_window.emit("login-data-captured", login_data)
            .map_err(|e| format!("发送事件失败: {}", e))?;
        println!("已发送数据到主窗口");
    }
    
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            open_login_window,
            capture_login_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
