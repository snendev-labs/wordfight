use serde::{Deserialize, Serialize};

use bevy::prelude::*;

use crate::{Letter, Word};

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
    pub fn apply(&self, word: &mut Word) {
        match self {
            Append(letter) => {
                word.push(*letter);
            }
            Delete => {
                word.pop();
            }
        }
    }
}
