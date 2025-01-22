use copypasta::{ClipboardContext, ClipboardProvider};
use rdev::{Event, EventType, Key};
use serde::{Deserialize, Serialize};
use std::{cell::Cell, sync::Mutex};
use tauri::{
    webview::{PageLoadEvent, PageLoadPayload},
    AppHandle, Emitter, Listener, Manager, WebviewWindow,
};
use weapon::{DpsWithRunes, Weapon};

pub type State = Mutex<Option<Data>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub weapon: Weapon,
    pub elapsed: u128,
    pub runes: Vec<DpsWithRunes>,
}

pub fn listen_global_ctrl_c(handle: AppHandle) {
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

#[derive(Debug)]
#[allow(unused)]
pub enum Error {
    Clipboard(ClipboardError),
    Parse(parser::ParseError),
}
#[derive(Debug)]
#[allow(unused)]
pub enum ClipboardError {
    CouldNotInitializeClipboardContext(String),
    CouldNotGetClipboardContents(String),
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

fn blocking_get_updated_clipboard() -> Result<(String, u128), ClipboardError> {
    use std::{thread, time::Duration};

    let mut retries_remaining = 10;

    let try_get_clipboard = || {
        let max_waiting_millis = 50;
        let timeout = Duration::from_millis(max_waiting_millis);
        let poll_interval = Duration::from_millis(1);
        let start_time = std::time::Instant::now();

        let mut clipboard = ClipboardContext::new()
            .map_err(|err| ClipboardError::CouldNotInitializeClipboardContext(err.to_string()))?;
        let previous_contents = clipboard
            .get_contents()
            .map_err(|err| ClipboardError::CouldNotGetClipboardContents(err.to_string()))?;

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
    };

    loop {
        retries_remaining -= 1;

        match try_get_clipboard() {
            Ok(ok) => return Ok(ok),
            Err(err) => {
                println!("Clipboard error: {err:?}");

                if retries_remaining == 0 {
                    return Err(err);
                }

                std::thread::sleep(Duration::from_millis(3));
            }
        }
    }
}

pub fn handle_ctrl_c(handle: &AppHandle) -> Result<(), Error> {
    let (contents, elapsed) = blocking_get_updated_clipboard().map_err(Error::Clipboard)?;

    let weapon = parser::parse(&contents)
        .map_err(Error::Parse)?
        .into_weapon();

    let runes = weapon.with_different_runes();

    let data = Data {
        weapon,
        elapsed,
        runes,
    };

    *handle.state::<State>().lock().unwrap() = Some(data.clone());

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

pub fn attach_window_listeners(handle: &AppHandle) {
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
