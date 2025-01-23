use clipboard_flow::State;

mod clipboard_flow;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .manage::<State>(State::default())
        .setup(|app| {
            let handle = app.handle().clone();
            clipboard_flow::attach_window_listeners(&handle);
            std::thread::spawn(move || clipboard_flow::listen_global_ctrl_c(handle));

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
