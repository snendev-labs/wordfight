use gloo_worker::{HandlerId, Worker, WorkerScope};
use wasm_bindgen::prelude::*;

use bevy::{prelude::*, utils::HashSet};

use wordfight::{ActiveGameUpdate, WordFightPlugins};

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
    my_player: Entity,
    subscriptions: HashSet<HandlerId>,
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
        app.update();
        app.update();

        let events = app.world().resource::<Events<ActiveGameUpdate>>();
        let mut reader = events.get_reader();
        let update = reader.read(&events).last().unwrap();
        let my_player = update.player_left;

        let scope_clone = scope.clone();
        let trigger_update = Closure::new(move || {
            scope_clone.send_message(());
        });
        let interval = setInterval(&trigger_update, 10);
        Self {
            app,
            my_player,
            subscriptions: HashSet::default(),
            _trigger_update: trigger_update,
            _interval: Interval(interval),
        }
    }

    fn update(&mut self, scope: &WorkerScope<Self>, _: Self::Message) {
        log("Update".to_string());
        self.app.update();
        let events = self.app.world().resource::<Events<ActiveGameUpdate>>();
        let mut reader = events.get_reader();
        if let Some(update) = reader.read(&events).last() {
            for id in &self.subscriptions {
                scope.respond(
                    *id,
                    WorkerMessage::UpdateState(UpdateStateMessage {
                        left_word: update.left_word.to_string(),
                        right_word: update.left_word.to_string(),
                        arena_size: update.arena_size,
                    }),
                );
            }
        }
    }

    fn received(&mut self, _: &WorkerScope<Self>, message: Self::Input, id: HandlerId) {
        log(format!("Message received! {:?}", message));
        self.subscriptions.insert(id);
        let action: wordfight::Action = match message {
            AppMessage::AddLetter(letter) => wordfight::Action::Append(letter),
            AppMessage::Backspace => wordfight::Action::Delete,
        };
        self.app
            .world_mut()
            .send_event(action.made_by(self.my_player, wordfight::PlayerSide::Left));

        self.app.update();
    }
}

pub struct Interval(f64);

impl Drop for Interval {
    fn drop(&mut self) {
        clearInterval(self.0);
    }
}
