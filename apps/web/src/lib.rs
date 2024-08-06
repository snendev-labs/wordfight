use serde::{Deserialize, Serialize};

pub use wordfight::*;

mod worker;
pub use worker::*;

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub enum AppMessage {
    AddLetter(Letter),
    Backspace,
}

impl AppMessage {
    pub fn add_letter(letter: &str) -> Option<Self> {
        Letter::from_str(letter).map(|letter| Self::AddLetter(letter))
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
    pub left_word: String,
    pub right_word: String,
    pub arena_size: usize,
}
