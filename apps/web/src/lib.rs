use serde::{Deserialize, Serialize};

pub use wordfight::*;

mod worker;
pub use worker::*;

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub enum AppMessage {
    AddLetter(AddLetterMessage),
    Backspace(BackspaceMessage),
}

impl AppMessage {
    pub fn add_letter(letter: &str) -> Option<Self> {
        Letter::from_str(letter).map(|letter| Self::AddLetter(AddLetterMessage(letter)))
    }

    pub fn backspace() -> Self {
        Self::Backspace(BackspaceMessage)
    }
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct AddLetterMessage(Letter);

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct BackspaceMessage;

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub enum WorkerMessage {
    UpdateState(UpdateStateMessage),
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct UpdateStateMessage {
    left_word: String,
    right_word: String,
    arena_size: usize,
}
