use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_replicon::prelude::*;

use crate::{InGame, Letter};

#[derive(Debug)]
#[derive(Component, Deref, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Client(ClientId);
pub use bevy_replicon::prelude::ClientId;

impl Client {
    pub fn bundle(self) -> impl Bundle {
        (
            Replicated,
            Name::new(format!("Player {}", self.0.get())),
            self,
        )
    }
}

impl From<ClientId> for Client {
    fn from(id: ClientId) -> Self {
        Client(id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub enum PlayerSide {
    Left,
    Right,
}

impl PlayerSide {
    pub fn is_left(&self) -> bool {
        matches!(self, PlayerSide::Left)
    }

    pub fn is_right(&self) -> bool {
        matches!(self, PlayerSide::Right)
    }
}

impl std::ops::Not for PlayerSide {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component, Deref, DerefMut, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Score(usize);

#[derive(Clone, Debug, Default)]
#[derive(Component, Deref, DerefMut, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Word(Vec<Letter>);

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|letter| letter.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub side: PlayerSide,
    pub score: Score,
    pub word: Word,
    pub in_game: InGame,
}
