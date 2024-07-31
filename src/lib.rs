use serde::{Deserialize, Serialize};

use bevy::{ecs::entity::MapEntities, prelude::*, utils::EntityHashMap};
// use bevy_anyhow_alert::*;
use bevy_replicon::prelude::*;

mod arena;
pub use arena::*;
mod letters;
pub use letters::*;
mod player;
pub use player::*;

pub struct WordFightPlugin;

impl Plugin for WordFightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconPlugins);

        app.add_mapped_client_event::<ActionEvent>(ChannelKind::Ordered);

        app.add_systems(
            Update,
            (Self::receive_input_events,)
                .chain()
                .in_set(WordFightSystems),
        );

        app.replicate::<Player>()
            .replicate::<PlayerSide>()
            .replicate::<Arena>()
            .replicate::<Score>()
            .replicate_mapped::<InArena>();
    }
}

impl WordFightPlugin {
    fn receive_input_events(
        mut action_events: EventReader<FromClient<ActionEvent>>,
        mut players: Query<(&Player, &mut Score, &PlayerSide, &InArena)>,
        mut arenas: Query<&mut Arena>,
    ) {
        let mut actions_by_arena: EntityHashMap<Entity, [Option<Action>; 2]> = Default::default();
        for FromClient {
            client_id,
            event: action,
        } in action_events.read()
        {
            let Some((_, _, side, in_arena)) = players
                .get(action.actor)
                .ok()
                .filter(|(player, _, _, _)| ***player == *client_id)
            else {
                continue;
            };
            let side_index = side.to_index();
            if let Some(inputs) = actions_by_arena.get_mut(&**in_arena) {
                inputs[side_index] = Some(action.action);
            } else {
                let mut actions = [None; 2];
                actions[side_index] = Some(action.action);
                actions_by_arena.insert(**in_arena, actions);
            }
        }

        for (arena_entity, actions) in actions_by_arena {
            let Ok(mut arena) = arenas.get_mut(arena_entity) else {
                continue;
            };
            let Ok(strike) = arena.execute_actions(actions) else {
                continue;
            };
            if let Strike::Point(winning_side) = strike {
                if let Some((_, mut score, _, _)) =
                    players.iter_mut().find(|(_, _, side, in_arena)| {
                        ***in_arena == arena_entity && ***side == winning_side
                    })
                {
                    **score += 1;
                }
            }
            arena.clear();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct WordFightSystems;

#[derive(Clone, Copy, Debug, Reflect, Serialize, Deserialize)]
pub enum Action {
    Append(Letter),
    Delete,
    // SuperCollapse,
    // SuperExtend,
}

impl Action {
    pub fn made_by(self, entity: Entity) -> ActionEvent {
        ActionEvent {
            action: self,
            actor: entity,
        }
    }
}

#[derive(Clone, Copy, Debug, Event, Reflect, Serialize, Deserialize)]
pub struct ActionEvent {
    action: Action,
    actor: Entity,
}

impl MapEntities for ActionEvent {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.actor = mapper.map_entity(self.actor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::MinimalPlugins;

    const WORD: [Letter; 8] = [
        Letter::A,
        Letter::L,
        Letter::P,
        Letter::H,
        Letter::A,
        Letter::B,
        Letter::E,
        Letter::T,
    ];

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(WordFightPlugin);
        app
    }

    fn spawn_game(world: &mut World, size: usize) -> (Entity, Entity, Entity) {
        let arena = world.spawn(Arena::new(size)).id();
        let player_one = world
            .spawn(PlayerBundle {
                player: ClientId::SERVER.into(),
                side: ArenaSide::Left.into(),
                score: Score::default(),
                in_arena: arena.into(),
            })
            .id();
        let player_two = world
            .spawn(PlayerBundle {
                player: ClientId::SERVER.into(),
                side: ArenaSide::Right.into(),
                score: Score::default(),
                in_arena: arena.into(),
            })
            .id();
        (arena, player_one, player_two)
    }

    #[test]
    fn test_normal_contact() {
        let mut app = app();
        let size = 7;

        let (arena_entity, player_one, player_two) = spawn_game(app.world_mut(), size);
        // update to let spawns / etc flush
        app.update();

        let mut arenas = app.world_mut().query::<&mut Arena>();
        let mut arena = arenas.single_mut(app.world_mut());
        for letter in &WORD[0..3] {
            arena.add_letter(*letter, ArenaSide::Left);
            arena.add_letter(*letter, ArenaSide::Right);
        }
        // nothing should happen here, but update to prevent influencing tests of future mutations
        app.update();

        let arena = app.world().get::<Arena>(arena_entity).unwrap();
        let (left, right) = arena.word_sizes();
        assert_eq!(left, 3);
        assert_eq!(right, 3);
        let score = app.world().get::<Score>(player_one).unwrap();
        assert_eq!(**score, 0);
        let score = app.world().get::<Score>(player_two).unwrap();
        assert_eq!(**score, 0);

        app.world_mut()
            .send_event::<ActionEvent>(Action::Append(WORD[3]).made_by(player_one));
        // update twice to process the event through replicon
        app.update();
        app.update();
        let arena = app.world().get::<Arena>(arena_entity).unwrap();
        let (left, right) = arena.word_sizes();
        assert_eq!(left, 0);
        assert_eq!(right, 0);
        let score = app.world().get::<Score>(player_one).unwrap();
        assert_eq!(**score, 0);
        let score = app.world().get::<Score>(player_two).unwrap();
        assert_eq!(**score, 1);
    }

    #[test]
    fn test_full_input() {
        let mut app = app();
        let size = 7;

        let (arena_entity, player_one, player_two) = spawn_game(app.world_mut(), size);
        // update to let spawns / etc flush
        app.update();

        let mut arenas = app.world_mut().query::<&mut Arena>();
        let mut arena = arenas.single_mut(app.world_mut());
        for letter in &WORD[0..6] {
            arena.add_letter(*letter, ArenaSide::Left);
        }
        // nothing should happen here, but update to prevent influencing tests of future mutations
        app.update();

        let arena = app.world().get::<Arena>(arena_entity).unwrap();
        let (left, right) = arena.word_sizes();
        assert_eq!(left, 6);
        assert_eq!(right, 0);
        let score = app.world().get::<Score>(player_one).unwrap();
        assert_eq!(**score, 0);
        let score = app.world().get::<Score>(player_two).unwrap();
        assert_eq!(**score, 0);

        app.world_mut()
            .send_event::<ActionEvent>(Action::Append(WORD[6]).made_by(player_one));
        // update twice to process the event through replicon
        app.update();
        app.update();

        let arena = app.world().get::<Arena>(arena_entity).unwrap();
        eprintln!("{arena:?}");
        let (left, right) = arena.word_sizes();
        assert_eq!(left, 0);
        assert_eq!(right, 0);
        let score = app.world().get::<Score>(player_one).unwrap();
        assert_eq!(**score, 1);
        let score = app.world().get::<Score>(player_two).unwrap();
        assert_eq!(**score, 0);
    }

    // test behavior when two inputs arrive at the same time
    #[test]
    fn test_colliding_inputs() {
        let mut app = app();
        let size = 7;

        let (arena_entity, player_one, player_two) = spawn_game(app.world_mut(), size);
        // update to let spawns / etc flush
        app.update();

        let mut arenas = app.world_mut().query::<&mut Arena>();
        let mut arena = arenas.single_mut(app.world_mut());
        for letter in &WORD[0..3] {
            arena.add_letter(*letter, ArenaSide::Left);
            arena.add_letter(*letter, ArenaSide::Right);
        }
        // nothing should happen here, but update to prevent influencing tests of future mutations
        app.update();

        let arena = app.world().get::<Arena>(arena_entity).unwrap();
        let (left, right) = arena.word_sizes();
        assert_eq!(left, 3);
        assert_eq!(right, 3);
        let score = app.world().get::<Score>(player_one).unwrap();
        assert_eq!(**score, 0);
        let score = app.world().get::<Score>(player_two).unwrap();
        assert_eq!(**score, 0);

        app.world_mut()
            .send_event::<ActionEvent>(Action::Append(WORD[3]).made_by(player_one));

            app.world_mut()
            .send_event::<ActionEvent>(Action::Append(WORD[3]).made_by(player_two));
        // update twice to process the event through replicon
        app.update();
        app.update();
        let arena = app.world().get::<Arena>(arena_entity).unwrap();
        let (left, right) = arena.word_sizes();
        assert_eq!(left, 0);
        assert_eq!(right, 0);
        let score = app.world().get::<Score>(player_one).unwrap();
        assert_eq!(**score, 0);
        let score = app.world().get::<Score>(player_two).unwrap();
        assert_eq!(**score, 0);
    }

    // test that one user fully filling the input
    #[test]
    fn test_full_interation() {
        let mut app = app();
        let size = 7;

        let (arena_entity, player_one, player_two) = spawn_game(app.world_mut(), size);
        // update to let spawns / etc flush
        app.update();

        // add the first 3 letters of the word to left and then right each letter
        for (index, letter) in WORD[0..3].iter().enumerate() {
            // first left

            let arena = app.world().get::<Arena>(arena_entity).unwrap();
            let (left, right) = arena.word_sizes();
            assert_eq!(left, index);
            assert_eq!(right, index);
            let score = app.world().get::<Score>(player_one).unwrap();
            assert_eq!(**score, 0);
            let score = app.world().get::<Score>(player_two).unwrap();
            assert_eq!(**score, 0);
            
            app.world_mut()
                .send_event::<ActionEvent>(Action::Append(*letter).made_by(player_one));
            // update twice to process the event through replicon
            app.update();
            app.update();

            // then right

            let arena = app.world().get::<Arena>(arena_entity).unwrap();
            let (left, right) = arena.word_sizes();
            assert_eq!(left, index + 1);
            assert_eq!(right, index);
            let score = app.world().get::<Score>(player_one).unwrap();
            assert_eq!(**score, 0);
            let score = app.world().get::<Score>(player_two).unwrap();
            assert_eq!(**score, 0);
            
            app.world_mut()
                .send_event::<ActionEvent>(Action::Append(*letter).made_by(player_two));
            // update twice to process the event through replicon
            app.update();
            app.update();
        }

        let arena = app.world().get::<Arena>(arena_entity).unwrap();
        let (left, right) = arena.word_sizes();
        assert_eq!(left, 3);
        assert_eq!(right, 3);
        let score = app.world().get::<Score>(player_one).unwrap();
        assert_eq!(**score, 0);
        let score = app.world().get::<Score>(player_two).unwrap();
        assert_eq!(**score, 0);

        app.world_mut()
            .send_event::<ActionEvent>(Action::Append(WORD[3]).made_by(player_one));
        // update twice to process the event through replicon
        app.update();
        app.update();
        let arena = app.world().get::<Arena>(arena_entity).unwrap();
        let (left, right) = arena.word_sizes();
        assert_eq!(left, 0);
        assert_eq!(right, 0);
        let score = app.world().get::<Score>(player_one).unwrap();
        assert_eq!(**score, 0);
        let score = app.world().get::<Score>(player_two).unwrap();
        assert_eq!(**score, 1);
    }

}
