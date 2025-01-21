#![allow(unused)]

use copypasta::{ClipboardContext, ClipboardProvider};
use parser::Parsed;
use rdev::{Event, EventType, Key};
use std::{cell::Cell, time::Duration};
use tauri::{
    webview::{PageLoadEvent, PageLoadPayload},
    AppHandle, Emitter, Event as TauriEvent, Listener, Manager, WebviewWindow,
    WebviewWindowBuilder,
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
                // window.open_devtools();
            }

            let handle = app.handle().clone();

            std::thread::spawn(move || {
                let ctrl_pressed = Cell::new(false);
                let result = rdev::grab(move |event| {
                    callback(event, &ctrl_pressed, || {
                        if let Err(err) = handle_ctrl_c_pressed(&handle) {
                            handle.emit("clipboard_flow_error", format!("{err:?}"));
                            println!("{err:?}");
                        };
                    })
                });

                if let Err(error) = result {
                    println!("Error: {:?}", error)
                };
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

#[derive(Debug)]
pub enum ClipboardFlowError {
    CouldNotInitializeClipboardContext(String),
    CouldNotGetClipboardContents(String),
    Parse(parser::ParseError),
}

fn handle_ctrl_c_pressed(handle: &AppHandle) -> Result<(), ClipboardFlowError> {
    let contents = blocking_get_updated_clipboard()?;

    println!("Clipboard: {contents}");

    let parsed = parser::parse(&contents).map_err(ClipboardFlowError::Parse)?;
    let emit_processed_data = |window: &WebviewWindow, parsed: &Parsed| {
        window.emit("clipboard_flow_data", &parsed).unwrap()
    };

    match get_clipboard_window(handle) {
        Some(window) => {
            println!("Window exists. sending payload: {parsed:?}");
            emit_processed_data(&window, &parsed)
        }
        None => {
            create_clipboard_window(handle, move |window, _payload| {
                emit_processed_data(&window, &parsed)
            });
        }
    }

    Ok(())
}

fn create_clipboard_window<T: Fn(WebviewWindow, PageLoadPayload<'_>) + Send + Sync + 'static>(
    handle: &AppHandle,
    on_page_load_finished: T,
) -> WebviewWindow {
    tauri::WebviewWindowBuilder::new(
        handle,
        "TheUniqueLabel",
        tauri::WebviewUrl::App("/clipboard-flow".into()),
    )
    .always_on_top(true)
    .devtools(true)
    .on_page_load(move |window, payload| {
        window.open_devtools();
        match payload.event() {
            PageLoadEvent::Started => {
                println!("{} finished loading", payload.url());
            }
            PageLoadEvent::Finished => {
                println!("{} finished loading", payload.url());
                on_page_load_finished(window, payload);
            }
        }
    })
    .build()
    .unwrap()
}

fn get_clipboard_window(handle: &AppHandle) -> Option<WebviewWindow> {
    handle.get_webview_window("TheUniqueLabel")
}

fn blocking_get_updated_clipboard() -> Result<String, ClipboardFlowError> {
    use std::{thread, time::Duration};

    let mut clipboard = ClipboardContext::new()
        .map_err(|err| ClipboardFlowError::CouldNotInitializeClipboardContext(err.to_string()))?;

    let timeout = Duration::from_millis(500);
    let poll_interval = Duration::from_millis(50);
    let start_time = std::time::Instant::now();

    let mut previous_contents = clipboard.get_contents().unwrap_or_default(); // Fallback to empty string if clipboard is empty or inaccessible

    while start_time.elapsed() < timeout {
        if let Ok(current_contents) = clipboard.get_contents() {
            if current_contents != previous_contents {
                println!("ELAPSED: {:?}", start_time.elapsed().as_millis());
                return Ok(current_contents); // New content detected
            }
        }

        thread::sleep(poll_interval); // Wait before polling again
    }

    Ok(previous_contents)
}
