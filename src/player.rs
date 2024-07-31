use serde::{Deserialize, Serialize};

use bevy::{ecs::entity::MapEntities, prelude::*};
// use bevy_anyhow_alert::*;
use bevy_replicon::prelude::*;

use crate::ArenaSide;

#[derive(Debug, Component, Deref, Reflect, Deserialize, Serialize)]
pub struct Player(ClientId);

impl From<ClientId> for Player {
    fn from(id: ClientId) -> Self {
        Player(id)
    }
}

#[derive(Clone, Copy, Debug, Component, Deref, DerefMut, Reflect, Deserialize, Serialize)]
pub struct PlayerSide(ArenaSide);

impl From<ArenaSide> for PlayerSide {
    fn from(side: ArenaSide) -> Self {
        PlayerSide(side)
    }
}

#[derive(Clone, Copy, Debug, Default, Component, Deref, DerefMut, Reflect, Deserialize, Serialize)]
pub struct Score(usize);

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Component,
    Deref,
    DerefMut,
    Reflect,
    Deserialize,
    Serialize,
)]
pub struct InArena(Entity);

impl From<Entity> for InArena {
    fn from(arena: Entity) -> Self {
        InArena(arena)
    }
}

impl MapEntities for InArena {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        **self = mapper.map_entity(**self);
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub side: PlayerSide,
    pub score: Score,
    pub in_arena: InArena,
}
