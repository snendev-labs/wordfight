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
                    info!("Added {letter}, making {word}");
                } else {
                    info!(
                        "{word}{} is not in the dictionary",
                        letter.to_string().to_lowercase()
                    );
                }
            }
            Delete => {
                let removed_letter = word.pop();
                info!("Removed {removed_letter:?} from {word}");
            }
        }
    }
}
