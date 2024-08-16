use serde::{Deserialize, Serialize};

pub use wordfight::*;

mod worker;
pub use worker::*;

pub const SERVER_IP: Option<&str> = option_env!("SERVER_IP");
pub const SERVER_DEFAULT_IP: &str = "127.0.0.1";

pub const SERVER_ORIGIN: Option<&str> = option_env!("SERVER_ORIGIN");
pub const SERVER_DEFAULT_ORIGIN: &str = "http://localhost";

pub const SERVER_PORT: Option<&str> = option_env!("SERVER_PORT");
pub const SERVER_DEFAULT_PORT: &str = "7636";

pub const SERVER_TOKENS_PORT: Option<&str> = option_env!("SERVER_TOKENS_PORT");
pub const SERVER_DEFAULT_TOKENS_PORT: &str = "7637";

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub enum AppMessage {
    AddLetter(Letter),
    Backspace,
}

impl AppMessage {
    pub fn add_letter(letter: &str) -> Option<Self> {
        Letter::from_string(letter).map(Self::AddLetter)
    }
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub enum WorkerMessage {
    UpdateState(UpdateStateMessage),
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct UpdateStateMessage {
    pub my_side: PlayerSide,
    pub left_word: String,
    pub left_score: usize,
    pub right_word: String,
    pub right_score: usize,
    pub arena_size: usize,
}
