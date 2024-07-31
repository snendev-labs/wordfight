use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use thiserror::Error;

use bevy::{prelude::*, utils::HashMap};

use crate::{Action, Letter};

#[derive(Clone, Debug, Component, Reflect, Deserialize, Serialize)]
pub struct Arena {
    words: HashMap<ArenaSide, Vec<Letter>>,
    size: usize,
}

impl Arena {
    pub fn new(size: usize) -> Self {
        let mut words = HashMap::new();
        words.insert(ArenaSide::Left, Vec::with_capacity(size));
        words.insert(ArenaSide::Right, Vec::with_capacity(size));
        Self { words, size }
    }

    pub fn word(&self, side: ArenaSide) -> &Vec<Letter> {
        self.words.get(&side).unwrap()
    }

    pub fn add_letter(&mut self, letter: Letter, side: ArenaSide) {
        self.words.get_mut(&side).unwrap().push(letter);
    }

    pub fn remove_letter(&mut self, side: ArenaSide) -> Option<Letter> {
        self.words.get_mut(&side).unwrap().pop()
    }

    pub fn word_sizes(&self) -> (usize, usize) {
        (self.word(ArenaSide::Left).len(), self.word(ArenaSide::Right).len())
    }

    pub fn clear(&mut self) {
        for (_, words) in self.words.iter_mut() {
            words.clear();
        }
    }

    pub fn execute_actions(&mut self, actions: [Option<Action>; 2]) -> Result<Strike, ArenaError> {
        for (action, side) in actions.into_iter().zip([ArenaSide::Left, ArenaSide::Right]) {
            match action {
                Some(Action::Append(letter)) => {
                    self.add_letter(letter, side);
                }
                Some(Action::Delete) => {
                    self.remove_letter(side);
                }
                _ => {}
            }
        }

        let total_letters: usize = self.words.iter().map(|(_, word)| word.len()).sum();
        if total_letters > self.size {
            return Ok(Strike::Parry);
        } else if total_letters < self.size {
            return Err(ArenaError::NotInRange {
                left: self.words.get(&ArenaSide::Left).unwrap().len(),
                right: self.words.get(&ArenaSide::Right).unwrap().len(),
                total: self.size,
            });
        };

        let left_letter = self.words.get(&ArenaSide::Left).unwrap().last();
        let right_letter = self.words.get(&ArenaSide::Right).unwrap().last();
        
        match left_letter.cmp(&right_letter) {
            Ordering::Less => Ok(Strike::Point(ArenaSide::Right)),
            Ordering::Greater => Ok(Strike::Point(ArenaSide::Left)),
            // one of the two must be Some, so this cannot be the None == None case.
            Ordering::Equal => Ok(Strike::Parry),
        }
    }
}

pub enum Strike {
    Point(ArenaSide),
    Parry,
}

#[derive(Debug, Error)]
pub enum ArenaError {
    #[error("Player cursors {left} and {right} out of range on arena size: {total}")]
    NotInRange {
        left: usize,
        right: usize,
        total: usize,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Deserialize, Serialize)]
pub enum ArenaSide {
    Left,
    Right,
}

impl ArenaSide {
    pub fn is_left(&self) -> bool {
        matches!(self, ArenaSide::Left)
    }

    pub fn is_right(&self) -> bool {
        matches!(self, ArenaSide::Right)
    }

    pub fn to_index(&self) -> usize {
        match self {
            ArenaSide::Left => 0,
            ArenaSide::Right => 1,
        }
    }
}

impl std::ops::Not for ArenaSide {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}
