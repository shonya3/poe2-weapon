#![allow(non_snake_case)]

use std::{rc::Rc, thread::sleep, time::Duration};

use dioxus::prelude::*;
use futures_core::Stream;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri_wasm::api::event::listen;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub fn App() -> Element {
    let mut text = use_signal(String::new);
    let parsed = use_memo(move || match text.read().is_empty() {
        true => None,
        false => Some(parser::parse(&text.read())),
    });

    let runes = move || match parsed.read().as_ref() {
        Some(result) => match result {
            Ok(parsed) => Some(parsed.clone().into_weapon().with_different_runes()),
            Err(_) => None,
        },
        None => None,
    };
    let events = listen::<String>("text");

    dioxus::prelude::spawn(async move {
        let mut stream = events.await.unwrap();

        while let Some(event) = stream.next().await {
            text.set(event.payload);
        }
        dioxus::prelude::schedule_update();
    });

    rsx! {
    pre { "{text}" }

            pre {"{parsed.read():?}"}

            pre { "{runes():#?}" }
        }
}
