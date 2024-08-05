use ev::KeyboardEvent;
use gloo_worker::Spawnable;
use leptos::*;
use wasm_bindgen::prelude::*;

use wordfight_web::{AppMessage, BevyWorker};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

fn main() {
    console_error_panic_hook::set_once();

    let bridge = BevyWorker::spawner()
        .callback(move |message| {
            log(format!("{:?}", message));
        })
        .spawn("./worker.js");
    let bridge = Box::leak(Box::new(bridge));

    mount_to_body(|| {
        view! {
            <main>
                <Game handle_input=move |event: KeyboardEvent| {
                    // event_target_value is a Leptos helper function
                    // it functions the same way as event.target.value
                    // in JavaScript, but smooths out some of the typecasting
                    // necessary to make this work in Rust
                    log(format!("{}", event.key().as_str()));
                    if let Some(message) = match event.key().as_str() {
                        "Backspace" | "Delete" | "ArrowLeft" => Some(AppMessage::backspace()),
                        letter => AppMessage::add_letter(letter),
                    } {
                        bridge.send(message);
                    }
                } />
            </main>
        }
    })
}

#[component]
fn Game(handle_input: impl Fn(KeyboardEvent) + 'static) -> impl IntoView {
    view! {
        <div tabindex="1" on:keyup=handle_input>
            <p>Hello world!</p>
        </div>
    }
}
