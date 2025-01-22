use copypasta::{ClipboardContext, ClipboardProvider};
use parser::Parsed;
use rdev::{Event, EventType, Key};
use serde::{Deserialize, Serialize};
use std::{cell::Cell, sync::Mutex};
use tauri::{
    webview::{PageLoadEvent, PageLoadPayload},
    AppHandle, Emitter, Listener, Manager, WebviewWindow,
};

#[derive(Debug)]
#[allow(unused)]
pub enum Error {
    CouldNotInitializeClipboardContext(String),
    CouldNotGetClipboardContents(String),
    Parse(parser::ParseError),
}

pub type State = Mutex<Option<Data>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub parsed: Parsed,
    pub elapsed: u128,
}

impl Data {
    pub fn emit(&self, window: &WebviewWindow) {
        window.emit("clipboard-flow-data", &self).unwrap()
    }
}

pub fn create_window<T: Fn(WebviewWindow, PageLoadPayload<'_>) + Send + Sync + 'static>(
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
        PageLoadEvent::Started => {}
        PageLoadEvent::Finished => {
            println!("{} finished loading", payload.url());
            on_page_load_finished(window, payload);
        }
    })
    .build()
    .unwrap()
}

pub fn get_window(handle: &AppHandle) -> Option<WebviewWindow> {
    handle.get_webview_window("TheUniqueLabel")
}

fn blocking_get_updated_clipboard() -> Result<(String, u128), Error> {
    use std::{thread, time::Duration};

    let max_waiting_millis = 500;
    let timeout = Duration::from_millis(max_waiting_millis);
    let poll_interval = Duration::from_millis(1);
    let start_time = std::time::Instant::now();

    let mut clipboard = ClipboardContext::new()
        .map_err(|err| Error::CouldNotInitializeClipboardContext(err.to_string()))?;
    let previous_contents = clipboard
        .get_contents()
        .map_err(|err| Error::CouldNotGetClipboardContents(err.to_string()))?;

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

pub fn handle_ctrl_c(handle: &AppHandle) -> Result<(), Error> {
    let (contents, elapsed) = blocking_get_updated_clipboard()?;

    let parsed = parser::parse(&contents).map_err(Error::Parse)?;

    let lock_handle = handle.state::<State>();
    let data = Data {
        parsed: parsed.clone(),
        elapsed,
    };

    *lock_handle.lock().unwrap() = Some(data.clone());

    match get_window(handle) {
        Some(window) => data.emit(&window),
        None => {
            create_window(handle, move |window, _payload| {
                data.emit(&window);
            });
        }
    }

    Ok(())
}

pub fn attach_event_listeners(handle: &AppHandle) {
    let window = tauri::WebviewWindowBuilder::new(
        handle,
        "TheUniqueLabel",
        tauri::WebviewUrl::App("/clipboard-flow".into()),
    )
    .visible(false)
    .build()
    .unwrap();

    let handle = handle.clone();
    window.listen("clipboard-flow-ask-resend", move |_| {
        if let Some(window) = get_window(&handle) {
            println!("Resending data");
            if let Some(data) = handle.state::<State>().inner().lock().unwrap().as_ref() {
                data.emit(&window);
            }
        }
    });
    window.close().unwrap();
}

fn listen_keyboard<T: Fn()>(event: Event, ctrl_pressed: &Cell<bool>, on_ctrl_c: T) {
    match event.event_type {
        EventType::KeyPress(Key::ControlLeft) => {
            ctrl_pressed.set(true);
        }
        EventType::KeyPress(Key::KeyC) => {
            if ctrl_pressed.get() {
                on_ctrl_c();
            }
        }
        EventType::KeyRelease(Key::ControlLeft) => {
            ctrl_pressed.set(false);
        }
        _ => {}
    }
}

pub fn listen_ctrl_c(handle: AppHandle) {
    let ctrl_pressed = Cell::new(false);
    let result = rdev::listen(move |event| {
        listen_keyboard(event, &ctrl_pressed, || {
            let handle = handle.clone();
            std::thread::spawn(move || {
                if let Err(err) = handle_ctrl_c(&handle) {
                    println!("{err:#?}");
                };
            });
        });
    });

    if let Err(error) = result {
        println!("Error: {:?}", error)
    };
}
