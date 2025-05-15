use clipboard_flow::State;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Manager, RunEvent, WindowEvent,
};

mod clipboard_flow;
mod tray_window;
mod commands {
    #[tauri::command]
    pub async fn open_browser(url: String) {
        open::that(url).unwrap();
    }
}

pub fn run() {
    let can_exit = AtomicBool::new(false);

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::open_browser])
        .manage::<State>(State::default())
        .setup(|app| {
            add_tray(app);
            let handle = app.handle().clone();
            tray_window::create_tray_window(&handle);

            clipboard_flow::attach_window_listeners(&handle);
            std::thread::spawn(move || clipboard_flow::listen_global_ctrl_c(handle));

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                if can_exit
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    api.prevent_exit();
                }
            }
            // Don't let closing windows to dictate, when app should be terminated
            RunEvent::WindowEvent {
                label,
                event: WindowEvent::CloseRequested { api, .. },
                ..
            } => {
                println!("{label}: Close window event.");

                // Hide ClipboardFlow window (instead of closing it) for instant startups
                if label.as_str() == clipboard_flow::WindowLabel.as_str() {
                    if let Some(window) = clipboard_flow::get_window(app.app_handle()) {
                        api.prevent_close();
                        window.hide().unwrap();
                    }
                }

                can_exit.store(false, Ordering::SeqCst);
            }
            RunEvent::MenuEvent(menu_event) => {
                if menu_event.id().as_ref() == "exit" {
                    can_exit.store(true, Ordering::SeqCst);
                    app.exit(0);
                    println!("Exit menu item was clicked");
                };
            }
            _ => {}
        });
}

fn add_tray(app: &App) {
    let quit_i = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&quit_i]).unwrap();

    TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("PoE2 Weapon")
        .show_menu_on_left_click(true)
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)
        .unwrap();
}
