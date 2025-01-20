#![allow(unused)]

use copypasta::{ClipboardContext, ClipboardProvider};
use rdev::{Event, EventType, Key};
use std::{cell::Cell, time::Duration};
use tauri::{
    webview::PageLoadEvent, Emitter, Event as TauriEvent, Listener, Manager, WebviewWindowBuilder,
};
use tauri_plugin_opener::OpenerExt;

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
                    match event.event_type {
                        EventType::KeyPress(Key::ControlLeft) => {
                            ctrl_pressed.set(true);
                        }
                        EventType::KeyPress(Key::KeyC) => {
                            if ctrl_pressed.get() {
                                std::thread::sleep(Duration::from_millis(400));

                                let mut clipboard = ClipboardContext::new().unwrap();
                                match clipboard.get_contents() {
                                    Ok(contents) => match parser::parse(&contents) {
                                        Ok(parsed) => {
                                            let weapon = parsed.into_weapon();
                                            println!("{weapon:#?}");
                                            println!(
                                                "DPS: {}, PDPS: {}",
                                                weapon.dps(),
                                                weapon.phys_dps()
                                            );

                                            let webview_window = tauri::WebviewWindowBuilder::new(
                                                &handle,
                                                "TheUniqueLabel",
                                                tauri::WebviewUrl::App("/about".into()),
                                            )
                                            .always_on_top(true)
                                            .on_page_load(|window, payload| match payload.event() {
                                                PageLoadEvent::Started => {
                                                    println!("{} finished loading", payload.url());
                                                }
                                                PageLoadEvent::Finished => {
                                                    println!("{} finished loading", payload.url());
                                                    window.emit("ready", ());
                                                }
                                            })
                                            .build()
                                            .unwrap();

                                            // println!("{:?}", weapon.with_different_runes());

                                            std::thread::sleep(Duration::from_millis(1));

                                            // webview_window.listen("ready", move |_| {
                                            // });
                                        }
                                        Err(err) => println!("ERROR parsing weapon text: {err:#?}"),
                                    },
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
