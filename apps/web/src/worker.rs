use gloo_worker::{HandlerId, Worker, WorkerScope};
use wasm_bindgen::prelude::*;

use bevy::prelude::*;

use wordfight::WordFightPlugins;

use crate::{AppMessage, UpdateStateMessage, WorkerMessage};

// Use this to enable console logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearInterval(token: f64);
}

pub struct BevyWorker {
    app: App,
    _trigger_update: Closure<dyn FnMut()>,
    _interval: Interval,
}

impl Worker for BevyWorker {
    type Input = AppMessage;
    type Output = WorkerMessage;
    type Message = ();

    fn create(scope: &WorkerScope<Self>) -> Self {
        let mut app = App::new();
        app.add_plugins(WordFightPlugins);
        let scope_clone = scope.clone();
        let trigger_update = Closure::new(move || {
            scope_clone.send_message(());
        });
        let interval = setInterval(&trigger_update, 10);
        Self {
            app,
            _trigger_update: trigger_update,
            _interval: Interval(interval),
        }
    }

    fn update(&mut self, _: &WorkerScope<Self>, _: Self::Message) {
        log("Update".to_string());
        self.app.update();
    }

    fn received(&mut self, scope: &WorkerScope<Self>, message: Self::Input, id: HandlerId) {
        log(format!("Message received! {:?}", message));
        scope.respond(
            id,
            WorkerMessage::UpdateState(UpdateStateMessage {
                left_word: "".to_string(),
                right_word: "".to_string(),
                arena_size: 7,
            }),
        );
    }
}

pub struct Interval(f64);

impl Drop for Interval {
    fn drop(&mut self) {
        clearInterval(self.0);
    }
}
