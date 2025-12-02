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

    let window_for_timer = login_window.clone();
    let app_for_timer = app.clone();
    
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3));
        let mut attempts = 0;
        let max_attempts = 60;
        
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
            
            let capture_script = r#"
                new Promise((resolve) => {
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
                        
                        resolve({
                            token: token,
                            cookies: cookies,
                            url: window.location.href,
                            allData: JSON.stringify(allData),
                            timestamp: new Date().toISOString()
                        });
                    } catch (e) {
                        resolve({
                            error: e.toString(),
                            token: '',
                            cookies: document.cookie || '',
                            url: window.location.href,
                            allData: '{}',
                            timestamp: new Date().toISOString()
                        });
                    }
                });
            "#;
            
            match window_for_timer.eval(capture_script) {
                Ok(_) => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                    
                    if let Some(main_window) = app_for_timer.get_webview_window("main") {
                        let _ = main_window.emit("checking-login", ());
                    }
                },
                Err(e) => {
                    println!("捕获数据失败: {}", e);
                }
            }
        }
    });

    let app_for_script = app.clone();
    let window_for_script = login_window.clone();
    
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        let inject_script = r#"
            (function() {
                const originalSetItem = localStorage.setItem;
                localStorage.setItem = function(key, value) {
                    console.log('localStorage.setItem:', key, value);
                    
                    if (key.toLowerCase().includes('token') || 
                        key.toLowerCase().includes('auth') ||
                        key.toLowerCase().includes('session') ||
                        key.toLowerCase().includes('user')) {
                        
                        window.__TAURI_INVOKE__('capture_login_data', {
                            token: value,
                            cookies: document.cookie,
                            url: window.location.href,
                            source: 'localStorage.' + key,
                            timestamp: new Date().toISOString()
                        }).catch(e => console.error('Failed to send to Tauri:', e));
                    }
                    
                    return originalSetItem.apply(this, arguments);
                };
                
                setInterval(function() {
                    let token = '';
                    let allData = {};
                    
                    try {
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
                        
                        if (token || cookies) {
                            let dataStr = JSON.stringify(allData);
                            window.__TAURI_INVOKE__('capture_login_data', {
                                token: token,
                                cookies: cookies,
                                url: window.location.href,
                                allData: dataStr,
                                timestamp: new Date().toISOString()
                            }).catch(e => {});
                        }
                    } catch (e) {
                        console.error('Error in periodic check:', e);
                    }
                }, 3000);
                
                console.log('Tauri login capture script injected');
            })();
        "#;
        
        if let Err(e) = window_for_script.eval(inject_script) {
            println!("注入脚本失败: {}", e);
        } else {
            println!("已注入捕获脚本");
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
    println!("========== 捕获到登录数据 ==========");
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
    println!("===================================");
    
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
