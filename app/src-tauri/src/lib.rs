use copypasta::{ClipboardContext, ClipboardProvider};
use parser::Parsed;
use rdev::{Event, EventType, Key};
use serde::{Deserialize, Serialize};
use std::{cell::Cell, sync::Mutex};
use tauri::{
    webview::{PageLoadEvent, PageLoadPayload},
    AppHandle, Emitter, Listener, Manager, WebviewWindow,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClipboardFlowData {
    parsed: Parsed,
    elapsed: u128,
}

type ClipboardFlowState = Mutex<Option<ClipboardFlowData>>;

impl ClipboardFlowData {
    pub fn emit(&self, window: &WebviewWindow) {
        window.emit("clipboard-flow-data", &self).unwrap()
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .manage::<ClipboardFlowState>(Mutex::new(Option::<ClipboardFlowData>::None))
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
                    if let Some(data) = handle
                        .state::<ClipboardFlowState>()
                        .inner()
                        .lock()
                        .unwrap()
                        .as_ref()
                    {
                        data.emit(&window);
                    }
                }
            });
            clipboard_flow_window.close().unwrap();

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

    let lock_handle = handle.state::<ClipboardFlowState>();
    let data = ClipboardFlowData {
        parsed: parsed.clone(),
        elapsed,
    };

    *lock_handle.lock().unwrap() = Some(data.clone());

    match get_clipboard_window(handle) {
        Some(window) => data.emit(&window),
        None => {
            create_clipboard_window(handle, move |window, _payload| {
                data.emit(&window);
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
