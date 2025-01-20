#![allow(non_snake_case)]

use std::rc::Rc;

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

    let events = listen::<String>("text");

    wasm_bindgen_futures::spawn_local(async move {
        let mut stream = events.await.unwrap();

        while let Some(event) = stream.next().await {
            text.set(event.payload);
        }
    });

    rsx! {
        pre { "{text}" }
    }
}
