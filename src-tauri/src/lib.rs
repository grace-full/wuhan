use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager, Listener};
use tauri::webview::WebviewWindow;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct AuthData {
    token: Option<String>,
    cookies: Vec<CookieData>,
    url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct CookieData {
    name: String,
    value: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_auth_window(app: tauri::AppHandle) -> Result<(), String> {
    // Create a new window for authentication
    let auth_window = tauri::WebviewWindowBuilder::new(
        &app,
        "auth",
        tauri::WebviewUrl::External("https://example.com/login".parse().unwrap()),
    )
    .title("Login")
    .inner_size(800.0, 600.0)
    .build()
    .map_err(|e| e.to_string())?;

    // Set up navigation listener
    setup_navigation_listener(&auth_window)?;

    Ok(())
}

fn setup_navigation_listener(window: &WebviewWindow) -> Result<(), String> {
    let window_clone = window.clone();
    let app_handle = window.app_handle().clone();
    
    // Listen for navigation events
    window.on_navigation(move |url| {
        println!("Navigation detected: {}", url);
        
        // Check if this looks like a successful authentication
        // (You can customize this logic based on your auth flow)
        let url_str = url.to_string();
        
        if url_str.contains("token") || 
           url_str.contains("success") || 
           url_str.contains("callback") ||
           url_str.contains("access_token") {
            
            println!("Potential auth success detected!");
            
            // Extract token from URL if present
            let token = extract_token_from_url(&url_str);
            
            // Get cookies from the webview
            let window_for_eval = window_clone.clone();
            let app_for_emit = app_handle.clone();
            
            tauri::async_runtime::spawn(async move {
                // Execute JavaScript to get cookies and localStorage
                let cookies_result = window_for_eval.eval(
                    r#"
                    (function() {
                        try {
                            const cookies = document.cookie.split(';').map(c => {
                                const parts = c.trim().split('=');
                                return { name: parts[0], value: parts.slice(1).join('=') };
                            }).filter(c => c.name);
                            
                            const localStorage_data = {};
                            for (let i = 0; i < localStorage.length; i++) {
                                const key = localStorage.key(i);
                                localStorage_data[key] = localStorage.getItem(key);
                            }
                            
                            const sessionStorage_data = {};
                            for (let i = 0; i < sessionStorage.length; i++) {
                                const key = sessionStorage.key(i);
                                sessionStorage_data[key] = sessionStorage.getItem(key);
                            }
                            
                            return JSON.stringify({
                                cookies: cookies,
                                localStorage: localStorage_data,
                                sessionStorage: sessionStorage_data
                            });
                        } catch (e) {
                            return JSON.stringify({ error: e.message });
                        }
                    })();
                    "#
                );
                
                if let Ok(_) = cookies_result {
                    // Wait a bit for the JavaScript to execute
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
                
                // Send the captured data via IPC
                let auth_data = AuthData {
                    token: token.clone(),
                    cookies: vec![],
                    url: url_str.clone(),
                };
                
                // Emit event to main window
                if let Err(e) = app_for_emit.emit("auth-data-captured", auth_data) {
                    eprintln!("Failed to emit auth data: {}", e);
                }
            });
        }
        
        true // Allow navigation
    });
    
    Ok(())
}

fn extract_token_from_url(url: &str) -> Option<String> {
    // Try to extract token from URL parameters or hash
    if let Ok(parsed_url) = url::Url::parse(url) {
        // Check query parameters
        for (key, value) in parsed_url.query_pairs() {
            if key == "token" || key == "access_token" || key == "id_token" {
                return Some(value.to_string());
            }
        }
        
        // Check hash fragment
        if let Some(fragment) = parsed_url.fragment() {
            for param in fragment.split('&') {
                let parts: Vec<&str> = param.split('=').collect();
                if parts.len() == 2 {
                    if parts[0] == "token" || parts[0] == "access_token" || parts[0] == "id_token" {
                        return Some(parts[1].to_string());
                    }
                }
            }
        }
    }
    
    None
}

#[tauri::command]
async fn extract_auth_data_from_window(window: tauri::WebviewWindow) -> Result<String, String> {
    // Execute JavaScript to extract auth data
    let result = window.eval(
        r#"
        (async function() {
            try {
                const cookies = document.cookie.split(';').map(c => {
                    const parts = c.trim().split('=');
                    return { name: parts[0], value: parts.slice(1).join('=') };
                }).filter(c => c.name);
                
                const localStorage_data = {};
                for (let i = 0; i < localStorage.length; i++) {
                    const key = localStorage.key(i);
                    localStorage_data[key] = localStorage.getItem(key);
                }
                
                const sessionStorage_data = {};
                for (let i = 0; i < sessionStorage.length; i++) {
                    const key = sessionStorage.key(i);
                    sessionStorage_data[key] = sessionStorage.getItem(key);
                }
                
                return JSON.stringify({
                    cookies: cookies,
                    localStorage: localStorage_data,
                    sessionStorage: sessionStorage_data,
                    url: window.location.href
                });
            } catch (e) {
                return JSON.stringify({ error: e.message });
            }
        })();
        "#
    );
    
    match result {
        Ok(_) => Ok("Data extraction initiated".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            // Set up global event listeners
            let app_handle = app.handle().clone();
            
            // Listen for window-specific events
            app.listen("capture-auth-data", move |event| {
                println!("Received capture-auth-data event: {:?}", event);
                
                if let Some(window_label) = event.payload().strip_prefix("\"").and_then(|s| s.strip_suffix("\"")) {
                    if let Some(window) = app_handle.get_webview_window(window_label) {
                        let window_clone = window.clone();
                        let app_clone = app_handle.clone();
                        
                        tauri::async_runtime::spawn(async move {
                            match extract_auth_data_from_window(window_clone).await {
                                Ok(data) => {
                                    println!("Auth data extracted: {}", data);
                                    let _ = app_clone.emit("auth-extraction-result", data);
                                }
                                Err(e) => {
                                    eprintln!("Failed to extract auth data: {}", e);
                                    let _ = app_clone.emit("auth-extraction-error", e);
                                }
                            }
                        });
                    }
                }
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            open_auth_window,
            extract_auth_data_from_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
