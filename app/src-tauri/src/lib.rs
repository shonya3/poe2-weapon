use clipboard_flow::State;

mod clipboard_flow;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .manage::<State>(State::default())
        .setup(|app| {
            let handle = app.handle().clone();

            clipboard_flow::attach_event_listeners(&handle);
            std::thread::spawn(move || clipboard_flow::listen_ctrl_c(handle));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
