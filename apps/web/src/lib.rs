use serde::{Deserialize, Serialize};

pub use wordfight::*;

mod worker;
pub use worker::*;

pub const SERVER_IP: Option<&'static str> = option_env!("SERVER_IP");
pub const SERVER_DEFAULT_IP: &'static str = "127.0.0.1";

pub const SERVER_PORT: Option<&'static str> = option_env!("SERVER_PORT");
pub const SERVER_DEFAULT_PORT: &'static str = "7636";

pub const SERVER_TOKENS_PORT: Option<&'static str> = option_env!("SERVER_TOKENS_PORT");
pub const SERVER_DEFAULT_TOKENS_PORT: &'static str = "7637";

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub enum AppMessage {
    AddLetter(Letter),
    Backspace,
}

impl AppMessage {
    pub fn add_letter(letter: &str) -> Option<Self> {
        Letter::from_string(letter).map(|letter| Self::AddLetter(letter))
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
