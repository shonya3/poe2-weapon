use copypasta::{ClipboardContext, ClipboardProvider};
use rdev::{Event, EventType, Key};
use std::{cell::Cell, time::Duration};
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            let handle = app.handle().clone();
            std::thread::spawn(move || {
                let ctrl_pressed = Cell::new(false);
                if let Err(error) = rdev::grab(move |event| {
                    // if event.event_type == EventType::KeyPress(Key::KeyD) {
                    //     simulate_ctrl_c();
                    //     std::thread::sleep(Duration::from_millis(20));

                    //     let mut clipboard = ClipboardContext::new().unwrap();
                    //     match clipboard.get_contents() {
                    //         Ok(contents) => println!("{contents}"),
                    //         Err(err) => eprintln!("{err}"),
                    //     }
                    // }

                    match event.event_type {
                        EventType::KeyPress(Key::ControlLeft) => {
                            ctrl_pressed.set(true);
                        }
                        EventType::KeyPress(Key::KeyC) => {
                            if ctrl_pressed.get() {
                                std::thread::sleep(Duration::from_millis(400));

                                let mut clipboard = ClipboardContext::new().unwrap();
                                match clipboard.get_contents() {
                                    Ok(contents) => println!("{contents}"),
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

                    // callback(event, &ctrl_pressed, || {
                    //     std::thread::sleep(Duration::from_millis(300));
                    //     println!("We are here");
                    //     let mud
                    //     // .build()
                    //     // .unwrap();
                    //     // println!("{webview_window:?}");
                    // })
                }) {
                    println!("Error: {:?}", error)
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn callback<T: Fn()>(
    event: Event,
    ctrl_pressed: &Cell<bool>,
    on_ctrl_c_pressed: T,
) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::ControlLeft) => {
            ctrl_pressed.set(true);
        }
        EventType::KeyPress(Key::KeyC) => {
            if ctrl_pressed.get() {
                on_ctrl_c_pressed();
            }
        }
        EventType::KeyRelease(Key::ControlLeft) => {
            ctrl_pressed.set(false);
        }
        _ => {}
    }

    // Always return the event to ensure the system gets it
    Some(event)
}

fn listen_keyboard<T: Fn()>(event: Event, key: Key, callback: T) -> Option<Event> {
    if event.event_type == EventType::KeyPress(key) {
        callback();
    }

    Some(event)
}

fn simulate_ctrl_c() {
    rdev::simulate(&EventType::KeyPress(Key::ControlLeft)).unwrap();
    std::thread::sleep(Duration::from_millis(2)); // Short delay
    rdev::simulate(&EventType::KeyPress(Key::KeyC)).unwrap();
    std::thread::sleep(Duration::from_millis(2));

    rdev::simulate(&EventType::KeyRelease(Key::KeyC)).unwrap();
    std::thread::sleep(Duration::from_millis(2)); // Short delay
    rdev::simulate(&EventType::KeyRelease(Key::ControlLeft)).unwrap();
}
