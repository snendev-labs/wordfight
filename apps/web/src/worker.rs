use gloo_worker::{HandlerId, Worker, WorkerScope};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use bevy::{prelude::*, utils::HashSet};

use client::ClientPlugin;
use wordfight::{ActiveGameUpdate, PlayerSide, WordFightPlugins};

use crate::{
    AppMessage, UpdateStateMessage, WorkerMessage, SERVER_DEFAULT_IP, SERVER_DEFAULT_PORT,
    SERVER_DEFAULT_TOKENS_PORT, SERVER_IP, SERVER_PORT, SERVER_TOKENS_PORT,
};

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
    game: Option<App>,
    subscriptions: HashSet<HandlerId>,
    _trigger_update: Closure<dyn FnMut()>,
    _interval: Interval,
}

impl Worker for BevyWorker {
    type Input = AppMessage;
    type Output = WorkerMessage;
    type Message = WorkerUpdateMessage;

    fn create(scope: &WorkerScope<Self>) -> Self {
        scope
            .send_future(async { WorkerUpdateMessage::Token(fetch_server_token().await.unwrap()) });
        let scope_clone = scope.clone();
        let trigger_update = Closure::new(move || {
            scope_clone.send_message(WorkerUpdateMessage::Update);
        });
        let interval = setInterval(&trigger_update, 10);
        Self {
            game: None,
            subscriptions: HashSet::default(),
            _trigger_update: trigger_update,
            _interval: Interval(interval),
        }
    }

    fn update(&mut self, scope: &WorkerScope<Self>, message: Self::Message) {
        #[cfg(feature = "log")]
        log("Update".to_string());
        if let Some(app) = self.game.as_mut() {
            let WorkerUpdateMessage::Update = message else {
                return;
            };
            app.update();
            let events = app.world().resource::<Events<ActiveGameUpdate>>();
            let mut reader = events.get_reader();
            if let Some(update) = reader.read(&events).last() {
                for id in &self.subscriptions {
                    scope.respond(
                        *id,
                        WorkerMessage::UpdateState(UpdateStateMessage {
                            left_word: update.left_word.to_string(),
                            left_score: *update.left_score,
                            right_word: update.right_word.to_string(),
                            right_score: *update.right_score,
                            arena_size: update.arena_size,
                        }),
                    );
                }
            }
        } else if let WorkerUpdateMessage::Token(token) = message {
            let app = build_app(token);
            self.game = Some(app);
        }
    }

    fn received(&mut self, _: &WorkerScope<Self>, message: Self::Input, id: HandlerId) {
        #[cfg(feature = "log")]
        log(format!("Message received! {:?}", message));
        if let Some(app) = self.game.as_mut() {
            self.subscriptions.insert(id);
            let action: wordfight::Action = match message {
                AppMessage::AddLetter(letter) => wordfight::Action::Append(letter),
                AppMessage::Backspace => wordfight::Action::Delete,
            };
            let mut query = app.world_mut().query::<(Entity, &PlayerSide)>();
            let Some((my_player, _)) = query
                .iter(app.world())
                .find(|(_, side)| **side == PlayerSide::Left)
            else {
                return;
            };
            app.world_mut()
                .send_event(action.made_by(my_player, wordfight::PlayerSide::Left));

            app.update();
        }
    }
}

pub enum WorkerUpdateMessage {
    Token(String),
    Update,
}

pub struct Interval(f64);

impl Drop for Interval {
    fn drop(&mut self) {
        clearInterval(self.0);
    }
}

fn build_app(server_token: String) -> App {
    let mut app = App::new();
    let server_origin = SERVER_IP.unwrap_or(SERVER_DEFAULT_IP).to_string();
    let server_port = SERVER_PORT.unwrap_or(SERVER_DEFAULT_PORT).to_string();

    app.add_plugins(WordFightPlugins);
    app.add_plugins(ClientPlugin {
        server_origin,
        server_port,
        server_token,
    });
    app.update();
    app.update();
    app
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn fetch(input: &Request) -> Promise;
}

async fn fetch_server_token() -> Result<String, JsValue> {
    let server_origin = SERVER_IP.unwrap_or(SERVER_DEFAULT_IP);
    let server_token_port = SERVER_TOKENS_PORT.unwrap_or(SERVER_DEFAULT_TOKENS_PORT);
    let server_url = format!("http://{server_origin}:{server_token_port}");
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&server_url, &opts)?;

    let response = JsFuture::from(fetch(&request)).await?;

    assert!(response.is_instance_of::<Response>());
    let response: Response = response.dyn_into().unwrap();
    let text = JsFuture::from(response.text()?).await?;

    Ok(text.as_string().unwrap())
}
