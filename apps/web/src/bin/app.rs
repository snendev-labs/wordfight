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

    let (left_word, set_left_word) = create_signal("".to_string());
    let (right_word, set_right_word) = create_signal("".to_string());
    let (arena_size, set_arena_size) = create_signal(0);

    let bridge = BevyWorker::spawner()
        .callback(move |message| {
            log(format!("Update from Bevy app: {:?}", message));
            match message {
                wordfight_web::WorkerMessage::UpdateState(state) => {
                    set_left_word.set(state.left_word);
                    set_right_word.set(state.right_word);
                    set_arena_size.set(state.arena_size);
                }
            }
        })
        .spawn("./worker.js");
    let bridge = Box::leak(Box::new(bridge));

    mount_to_body(move || {
        view! {
            <main>
                <Game
                left_word=left_word.get()
                right_word=right_word.get()
                arena_size=arena_size.get()
                handle_input=move |event: KeyboardEvent| {
                    // event_target_value is a Leptos helper function
                    // it functions the same way as event.target.value
                    // in JavaScript, but smooths out some of the typecasting
                    // necessary to make this work in Rust
                    log(format!("{}", event.key().as_str()));
                    if let Some(message) = match event.key().as_str() {
                        "Backspace" | "Delete" | "ArrowLeft" => Some(AppMessage::Backspace),
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
fn Game(
    left_word: String,
    right_word: String,
    arena_size: usize,
    handle_input: impl Fn(KeyboardEvent) + 'static,
) -> impl IntoView {
    log(left_word);
    log(right_word);
    log(format!("{arena_size}"));
    view! {
        <div class="center" tabindex="1" on:keyup=handle_input>
            <p>Hello world!</p>
        </div>
    }
}
