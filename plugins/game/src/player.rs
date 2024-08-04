use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_replicon::prelude::*;

use crate::Letter;

#[derive(Debug)]
#[derive(Component, Deref, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Client(ClientId);

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

#[derive(Bundle)]
pub struct PlayerBundle {
    pub client: Client,
    pub side: PlayerSide,
    pub score: Score,
    pub word: Word,
}
