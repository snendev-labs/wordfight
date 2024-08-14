use ev::KeyboardEvent;
use gloo_worker::Spawnable;
use leptos::*;
#[cfg(feature = "log")]
use wasm_bindgen::prelude::*;

use wordfight_web::{AppMessage, BevyWorker};

#[cfg(feature = "log")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(move || {
        view! {
            <main>
                <App />
            </main>
        }
    })
}

#[component]
fn App() -> impl IntoView {
    let (left_word, set_left_word) = create_signal("".to_string());
    let (left_score, set_left_score) = create_signal(0);
    let (right_word, set_right_word) = create_signal("".to_string());
    let (right_score, set_right_score) = create_signal(0);
    let (arena_size, set_arena_size) = create_signal(7);
    #[cfg(feature = "log")]
    log("Render (App)".to_string());

    let bridge = BevyWorker::spawner()
        .callback(move |message| {
            #[cfg(feature = "log")]
            log(format!("Update from Bevy app: {:?}", message));
            match message {
                wordfight_web::WorkerMessage::UpdateState(state) => {
                    #[cfg(feature = "log")]
                    log(format!("Setting state..."));
                    set_left_word.set(state.left_word);
                    set_left_score.set(state.left_score);
                    set_right_word.set(state.right_word);
                    set_right_score.set(state.right_score);
                    set_arena_size.set(state.arena_size);
                }
            }
        })
        .spawn("./worker.js");
    let bridge = Box::leak(Box::new(bridge));

    let handle_input = move |event: KeyboardEvent| {
        if let Some(message) = match event.key().as_str() {
            "Backspace" | "Delete" | "ArrowLeft" => Some(AppMessage::Backspace),
            letter => AppMessage::add_letter(letter),
        } {
            bridge.send(message);
        }
    };

    view! {
        <div class="center" tabindex="1" on:keyup=handle_input>
            <Game
                left_word=left_word
                left_score=left_score
                right_word=right_word
                right_score=right_score
                arena_size=arena_size
            />
        </div>
    }
}

#[component]
fn Game(
    left_word: ReadSignal<String>,
    left_score: ReadSignal<usize>,
    right_word: ReadSignal<String>,
    right_score: ReadSignal<usize>,
    arena_size: ReadSignal<usize>,
) -> impl IntoView {
    #[cfg(feature = "log")]
    log("Render (Game)".to_string());
    view! {
        <div>
            "Arena: "
            {arena_size}
        </div>
        <Scoreboard left_score=left_score right_score=right_score />
        <div class="arena">
            <Word top_word=left_word bottom_word=right_word arena_size=arena_size />
            <Word top_word=right_word bottom_word=left_word arena_size=arena_size />
        </div>
    }
}

#[component]
fn Scoreboard(left_score: ReadSignal<usize>, right_score: ReadSignal<usize>) -> impl IntoView {
    #[cfg(feature = "log")]
    log("Render (Scoreboard)".to_string());
    view! {
        <div class="scoreboard">
            <div>{left_score}</div>
            <hr/>
            <div>{right_score}</div>
        </div>
    }
}

#[component]
fn Word(
    top_word: ReadSignal<String>,
    bottom_word: ReadSignal<String>,
    arena_size: ReadSignal<usize>,
) -> impl IntoView {
    #[cfg(feature = "log")]
    log("Render (Word)".to_string());
    let empty_spaces_count = move || {
        arena_size
            .get()
            .saturating_sub(top_word.get().len())
            .saturating_sub(bottom_word.get().len())
    };
    let overwritten_spaces_count =
        move || (top_word.get().len() + bottom_word.get().len()).saturating_sub(arena_size.get());
    let total_word = move ||
        // start with the top word
        top_word
            .get()
            .chars()
            .map(|char| Some(char))
            // then include the number of empty spaces
            .chain((0..empty_spaces_count()).map(|_| None))
            // then include the bottom word, reversed, after skipping any overwritten letters
            .chain(
                bottom_word.get()
                    .chars()
                    .rev()
                    .skip(overwritten_spaces_count())
                    .map(|char| Some(char)),
            )
            .enumerate()
            .collect::<Vec<_>>();
    view! {
        <div class="arena-column">
            <For
                each=total_word
                // aggressively recompute!
                // it is fine to use the index in most cases because the total arena
                // size shouldn't change except in rare circumstances where we do want
                // a full rerender
                key=|(index, maybe_char)| { format!("{}{:?}", index, maybe_char)}
                children=move |(_, maybe_char)| {
                    view! {
                        <Letter letter=maybe_char />
                    }
                }
            />
        </div>
    }
}

#[component]
fn Letter(letter: Option<char>) -> impl IntoView {
    #[cfg(feature = "log")]
    log("Render (Letter)".to_string());
    view! {
        <div class="letter-slot">
            {letter}
        </div>
    }
}
