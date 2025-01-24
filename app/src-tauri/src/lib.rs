use clipboard_flow::State;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, AppHandle, RunEvent, WindowEvent,
};

mod clipboard_flow;
mod commands {
    #[tauri::command]
    pub async fn open_browser(url: String) {
        open::that(url).unwrap();
    }
}

pub fn run() {
    let can_exit = AtomicBool::new(false);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::open_browser])
        .manage::<State>(State::default())
        .setup(|app| {
            add_tray(app);
            let handle = app.handle().clone();

            clipboard_flow::attach_window_listeners(&handle);
            std::thread::spawn(move || clipboard_flow::listen_global_ctrl_c(handle));

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |_, event| 
            // Don't let closing windows to dictate, when app should be terminated
            match event {
            RunEvent::ExitRequested { api, .. } => {
                if can_exit
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    api.prevent_exit();
                }
            }

            RunEvent::WindowEvent {
                label,
                event: WindowEvent::CloseRequested { .. },
                ..
            } => {
                println!("{label}: Close window event.");
                can_exit.store(false, Ordering::SeqCst);
            }
            _ => {}
        });
}

fn add_tray(app: &App) {
    let quit_i = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&quit_i]).unwrap();

    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("PoE2 Weapon")
        .show_menu_on_left_click(true)
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)
        .unwrap();

    tray.on_menu_event(|app: &AppHandle, event| match event.id.as_ref() {
        "exit" => {
            println!("Exit menu item was clicked");
            app.exit(0);
        }
        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    });
}

