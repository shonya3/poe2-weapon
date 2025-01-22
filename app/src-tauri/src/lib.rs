#![allow(unused)]

use copypasta::{ClipboardContext, ClipboardProvider};
use parser::Parsed;
use rdev::{Event, EventType, Key};
use serde::{Deserialize, Serialize};
use std::{
    cell::Cell,
    sync::{Arc, Mutex},
    time::Duration,
};
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

struct ClipboardFlowState {
    parsed: Option<ParsedElapsed>,
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .manage(Mutex::new(ClipboardFlowState { parsed: None }))
        .setup(|app| {
            let handle = app.handle().clone();
            let main_window = tauri::WebviewWindowBuilder::new(
                &handle,
                "main",
                tauri::WebviewUrl::App("/".into()),
            )
            .build()
            .unwrap();
            #[cfg(debug_assertions)]
            {
                main_window.open_devtools();
            }

            let clipboard_flow_window = tauri::WebviewWindowBuilder::new(
                &handle,
                "TheUniqueLabel",
                tauri::WebviewUrl::App("/clipboard-flow".into()),
            )
            .visible(false)
            .build()
            .unwrap();

            let handle = app.handle().clone();
            clipboard_flow_window.listen("clipboard-flow-ask-resend", move |_| {
                if let Some(window) = get_clipboard_window(&handle) {
                    println!("Resending data");
                    let state = handle
                        .state::<Mutex<ClipboardFlowState>>()
                        .inner()
                        .lock()
                        .unwrap();
                    window.emit("clipboard-flow-data", &state.parsed).unwrap();
                }
            });
            clipboard_flow_window.close();

            let handle = app.handle().clone();
            std::thread::spawn(move || {
                let ctrl_pressed = Cell::new(false);
                let result = rdev::listen(move |event| {
                    callback(event, &ctrl_pressed, || {
                        let handle = handle.clone();
                        std::thread::spawn(move || {
                            if let Err(err) = handle_ctrl_c_pressed(&handle) {
                                println!("{err:#?}");
                            };
                        });
                    });
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

fn callback<T: Fn()>(event: Event, ctrl_pressed: &Cell<bool>, on_ctrl_c_pressed: T) {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ParsedElapsed {
    pub elapsed: u128,
    pub parsed: Parsed,
}

fn handle_ctrl_c_pressed(handle: &AppHandle) -> Result<(), ClipboardFlowError> {
    let (contents, elapsed) = blocking_get_updated_clipboard()?;

    let parsed = parser::parse(&contents).map_err(ClipboardFlowError::Parse)?;
    let emit_processed_data = |window: &WebviewWindow, parsed: &ParsedElapsed| {
        window.emit("clipboard-flow-data", &parsed).unwrap();
    };

    let parsedelapsed = ParsedElapsed { elapsed, parsed };
    let lock_handle = handle.state::<Mutex<ClipboardFlowState>>();
    lock_handle.lock().unwrap().parsed = Some(parsedelapsed.clone());

    let handle_clone = handle.clone();
    match get_clipboard_window(handle) {
        Some(window) => emit_processed_data(&window, &parsedelapsed),
        None => {
            create_clipboard_window(handle, move |window, _payload| {
                emit_processed_data(&window, &parsedelapsed);
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
    .on_page_load(move |window, payload| match payload.event() {
        PageLoadEvent::Started => {
            println!("{} finished loading", payload.url());
        }
        PageLoadEvent::Finished => {
            println!("{} finished loading", payload.url());
            on_page_load_finished(window, payload);
        }
    })
    .build()
    .unwrap()
}

fn get_clipboard_window(handle: &AppHandle) -> Option<WebviewWindow> {
    handle.get_webview_window("TheUniqueLabel")
}

fn blocking_get_updated_clipboard() -> Result<(String, u128), ClipboardFlowError> {
    use std::{thread, time::Duration};

    let max_waiting_millis = 500;
    let timeout = Duration::from_millis(max_waiting_millis);
    let poll_interval = Duration::from_millis(1);
    let start_time = std::time::Instant::now();

    let mut clipboard = ClipboardContext::new()
        .map_err(|err| ClipboardFlowError::CouldNotInitializeClipboardContext(err.to_string()))?;
    let previous_contents = clipboard
        .get_contents()
        .map_err(|err| ClipboardFlowError::CouldNotGetClipboardContents(err.to_string()))?;

    while start_time.elapsed() < timeout {
        if let Ok(current_contents) = clipboard.get_contents() {
            if current_contents != previous_contents {
                println!("ELAPSED: {:?}", start_time.elapsed().as_millis());
                return Ok((current_contents, start_time.elapsed().as_millis()));
            }
        }

        thread::sleep(poll_interval);
    }

    Ok((previous_contents, max_waiting_millis as u128))
}
