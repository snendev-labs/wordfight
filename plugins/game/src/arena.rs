use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use thiserror::Error;

use bevy::{ecs::entity::MapEntities, prelude::*};

use crate::{PlayerSide, Word};

#[derive(Bundle)]
pub struct GameBundle {
    game: Game,
    players: GamePlayers,
    arena: Arena,
}

impl GameBundle {
    pub fn new(left: Entity, right: Entity, arena_size: usize) -> Self {
        GameBundle {
            game: Game,
            players: GamePlayers { left, right },
            arena: Arena::new(arena_size),
        }
    }
}

#[derive(Clone, Debug)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Game;

#[derive(Clone, Debug)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct GamePlayers {
    pub left: Entity,
    pub right: Entity,
}

impl MapEntities for GamePlayers {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.left = mapper.map_entity(self.left);
        self.right = mapper.map_entity(self.right);
    }
}

#[derive(Clone, Debug)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Arena {
    size: usize,
}

impl Arena {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn strike(&self, left_word: &Word, right_word: &Word) -> Result<Strike, ArenaError> {
        let total_letters = left_word.len() + right_word.len();
        if total_letters > self.size {
            return Ok(Strike::OverRange);
        } else if total_letters < self.size {
            return Err(ArenaError::NotInRange {
                left: left_word.len(),
                right: right_word.len(),
                total: self.size,
            });
        };

        match left_word.last().cmp(&right_word.last()) {
            Ordering::Less => Ok(Strike::Score(PlayerSide::Right)),
            Ordering::Greater => Ok(Strike::Score(PlayerSide::Left)),
            // one of the two must be Some, so this cannot be the None == None case.
            Ordering::Equal => Ok(Strike::Parry),
        }
    }
}

pub enum Strike {
    Score(PlayerSide),
    Parry,
    OverRange,
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
