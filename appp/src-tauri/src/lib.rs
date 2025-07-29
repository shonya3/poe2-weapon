use std::{cell::Cell, time::Duration};

use copypasta::{ClipboardContext, ClipboardProvider};
use rdev::{EventType, Key};
use tauri::{webview::PageLoadEvent, Emitter, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                // if let Some(window) = app.get_webview_window("main") {
                // window.open_devtools();
                // window.hide().unwrap();
                // };
            }

            let handle = app.handle().clone();
            std::thread::spawn(move || {
                let ctrl_pressed = Cell::new(false);
                if let Err(error) = rdev::grab(move |event| {
                    match event.event_type {
                        EventType::KeyPress(Key::ControlLeft) => {
                            ctrl_pressed.set(true);
                        }
                        EventType::KeyPress(Key::KeyC) => {
                            if ctrl_pressed.get() {
                                std::thread::sleep(Duration::from_millis(400));

                                let mut clipboard = ClipboardContext::new().unwrap();
                                match clipboard.get_contents() {
                                    Ok(contents) => {
                                        if let Some(window) =
                                            handle.get_webview_window("ClipboardFlowWindow")
                                        {
                                            let _ = window.emit("text", contents.clone());
                                            println!("Emitted text");
                                        } else {
                                            tauri::WebviewWindowBuilder::new(
                                                &handle,
                                                "ClipboardFlowWindow",
                                                tauri::WebviewUrl::App("/".into()),
                                            )
                                            .always_on_top(true)
                                            .on_page_load(move |window, payload| {
                                                window.open_devtools();
                                                match payload.event() {
                                                    PageLoadEvent::Started => {
                                                        println!(
                                                            "{} finished loading",
                                                            payload.url()
                                                        );
                                                    }
                                                    PageLoadEvent::Finished => {
                                                        println!(
                                                            "{} finished loading",
                                                            payload.url()
                                                        );
                                                        let _ =
                                                            window.emit("text", contents.clone());
                                                        println!("Emitted text");
                                                    }
                                                }
                                            })
                                            .build()
                                            .unwrap();
                                        }
                                    }
                                    Err(err) => eprintln!("{err}"),
                                }
                            }
                        }
                        EventType::KeyRelease(Key::ControlLeft) => {
                            ctrl_pressed.set(false);
                        }
                        _ => {}
                    }

                    Some(event)
                }) {
                    println!("Error: {error:?}")
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
