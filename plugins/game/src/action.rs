use serde::{Deserialize, Serialize};

use bevy::prelude::*;

use crate::{Dictionary, Letter, Word};

#[derive(Clone, Copy, Debug)]
#[derive(Reflect)]
#[derive(Serialize, Deserialize)]
pub enum Action {
    Append(Letter),
    Delete,
    // SuperCollapse,
    // SuperExtend,
}
use Action::{Append, Delete};

impl Action {
    pub fn apply(&self, word: &mut Word, dictionary: &Dictionary) {
        match self {
            Append(letter) => {
                let test_string = format!("{}{}", word.to_string(), letter.to_string());
                if dictionary.is_word_substring(test_string.as_str()) {
                    word.push(*letter);
                }
            }
            Delete => {
                word.pop();
            }
        }
    }
}
