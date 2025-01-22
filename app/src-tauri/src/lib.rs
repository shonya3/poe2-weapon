use clipboard_flow::State;
use tauri::{Manager, WebviewWindow, WebviewWindowBuilder};

mod clipboard_flow;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .manage::<State>(State::default())
        .setup(|app| {
            let handle = app.handle().clone();
            WebviewWindowBuilder::new(&handle, "main", tauri::WebviewUrl::App("/".into()))
                .visible(false)
                .build()
                .unwrap();
            // handle.get_webview_window("main").unwrap().hide().unwrap();

            clipboard_flow::attach_window_listeners(&handle);
            std::thread::spawn(move || clipboard_flow::listen_global_ctrl_c(handle));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
