use serde::{Deserialize, Serialize};

use bevy::{ecs::entity::MapEntities, prelude::*};
use bevy_replicon::prelude::*;

mod action;
pub use action::*;
mod arena;
pub use arena::*;
mod letters;
pub use letters::*;
mod player;
pub use player::*;
mod wordlist;
pub use wordlist::*;

pub struct WordFightGamePlugin;

impl Plugin for WordFightGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconPlugins);

        // TODO: perhaps we only want to include this on server.
        // check whether it is currently "optimistic", if so, maybe we keep it
        app.init_resource::<WordList>();
        app.add_mapped_client_event::<ActionEvent>(ChannelKind::Ordered);

        app.add_systems(
            Update,
            (Self::handle_input_actions, Self::handle_word_contact)
                .chain()
                .in_set(WordFightSystems),
        );

        app.observe(SpawnGame::observer);

        app.replicate::<Client>()
            .replicate::<PlayerSide>()
            .replicate::<Word>()
            .replicate::<Score>()
            .replicate::<Game>()
            .replicate::<Arena>()
            .replicate_mapped::<GamePlayers>();
    }
}

impl WordFightGamePlugin {
    fn handle_input_actions(
        mut action_events: EventReader<FromClient<ActionEvent>>,
        mut players: Query<(&mut Word, &PlayerSide, &Client)>,
        dictionary: Dictionary,
    ) {
        for FromClient {
            client_id,
            event: action,
        } in action_events.read()
        {
            let ActionEvent {
                action,
                side,
                actor,
            } = action;
            let Ok((mut word, player_side, client)) = players.get_mut(*actor) else {
                continue;
            };

            if **client == *client_id && *player_side == *side {
                action.apply(&mut word, &dictionary);
            }
        }
    }

    fn handle_word_contact(
        mut players: Query<(&mut Word, &mut Score)>,
        arenas: Query<(&Arena, &GamePlayers)>,
    ) {
        for (arena, game_players) in &arenas {
            let Ok([(left_word, _), (right_word, _)]) =
                players.get_many([game_players.left, game_players.right])
            else {
                continue;
            };
            let Ok(strike) = arena.strike(left_word, right_word) else {
                continue;
            };
            // contact has occurred!
            // first determine whether anyone gets a point
            match strike {
                Strike::Score(winning_side) => {
                    let winner = match winning_side {
                        PlayerSide::Left => game_players.left,
                        PlayerSide::Right => game_players.right,
                    };
                    let Ok((_, mut score)) = players.get_mut(winner) else {
                        continue;
                    };
                    **score += 1;
                }
                // both parry conditions result in no score change
                Strike::OverRange | Strike::Parry => {}
            }
            // then clear both player words
            for (mut word, _) in players
                .get_many_mut([game_players.left, game_players.right])
                .into_iter()
                .flatten()
            {
                word.clear();
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(SystemSet)]
pub struct WordFightSystems;

impl Action {
    pub fn made_by(self, entity: Entity, side: PlayerSide) -> ActionEvent {
        ActionEvent {
            action: self,
            side,
            actor: entity,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(Event, Reflect)]
#[derive(Serialize, Deserialize)]
pub struct ActionEvent {
    action: Action,
    side: PlayerSide,
    actor: Entity,
}

impl MapEntities for ActionEvent {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.actor = mapper.map_entity(self.actor);
    }
}

#[derive(Debug)]
#[derive(Event)]
pub struct SpawnGame {
    arena_size: usize,
}

impl SpawnGame {
    pub fn new(arena_size: usize) -> Self {
        Self { arena_size }
    }

    fn observer(trigger: Trigger<Self>, mut commands: Commands) {
        eprintln!("Hello!");
        let player_one = commands
            .spawn(PlayerBundle {
                client: ClientId::SERVER.into(),
                side: PlayerSide::Left,
                word: Word::default(),
                score: Score::default(),
            })
            .id();
        let player_two = commands
            .spawn(PlayerBundle {
                client: ClientId::SERVER.into(),
                side: PlayerSide::Right,
                word: Word::default(),
                score: Score::default(),
            })
            .id();
        commands.spawn(GameBundle::new(
            player_one,
            player_two,
            trigger.event().arena_size,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::MinimalPlugins;

    const ALPHABET: [Letter; 8] = [
        Letter::A,
        Letter::B,
        Letter::C,
        Letter::D,
        Letter::E,
        Letter::F,
        Letter::G,
        Letter::H,
    ];

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(WordFightGamePlugin);
        app.update();
        app
    }

    fn find_players(world: &mut World) -> (Entity, Entity) {
        let mut players_query = world.query::<(Entity, &PlayerSide)>();
        let player_one = players_query
            .iter(world)
            .find_map(|(entity, side)| match side {
                PlayerSide::Left => Some(entity),
                _ => None,
            })
            .unwrap();
        let player_two = players_query
            .iter(world)
            .find_map(|(entity, side)| match side {
                PlayerSide::Right => Some(entity),
                _ => None,
            })
            .unwrap();
        (player_one, player_two)
    }

    fn set_word(world: &mut World, player: Entity, new_word: Vec<Letter>) {
        let mut word = world.get_mut::<Word>(player).unwrap();
        **word = new_word;
    }

    fn assert_word_sizes(world: &World, left: (Entity, usize), right: (Entity, usize)) {
        let left_word_len = world.get::<Word>(left.0).unwrap().len();
        assert_eq!(left_word_len, left.1);
        let right_word_len = world.get::<Word>(right.0).unwrap().len();
        assert_eq!(right_word_len, right.1);
    }

    fn assert_scores(world: &World, left: (Entity, usize), right: (Entity, usize)) {
        let left_score = world.get::<Score>(left.0).unwrap();
        assert_eq!(**left_score, left.1);
        let right_score = world.get::<Score>(right.0).unwrap();
        assert_eq!(**right_score, right.1);
    }

    // test the Strike::Score behavior in a typical scenario where one letter is larger than the other
    #[test]
    fn test_strike_score_typical() {
        let mut app = app();

        let size = 7;
        app.world_mut().trigger(SpawnGame::new(size));
        // update to let spawns / etc flush
        app.update();

        let (player_one, player_two) = find_players(app.world_mut());

        let first_three_letters: Vec<Letter> = ALPHABET[0..3].iter().cloned().collect();
        set_word(app.world_mut(), player_one, first_three_letters.clone());
        set_word(app.world_mut(), player_two, first_three_letters);

        // nothing should happen here, but update to prevent influencing tests of future mutations
        app.update();

        assert_word_sizes(app.world(), (player_one, 3), (player_two, 3));
        let score = app.world().get::<Score>(player_one).unwrap();
        assert_eq!(**score, 0);
        let score = app.world().get::<Score>(player_two).unwrap();
        assert_eq!(**score, 0);

        app.world_mut().send_event::<ActionEvent>(
            Action::Append(ALPHABET[3]).made_by(player_one, PlayerSide::Left),
        );
        // update twice to process the event through replicon
        app.update();
        app.update();

        assert_word_sizes(app.world(), (player_one, 0), (player_two, 0));
        assert_scores(app.world(), (player_one, 1), (player_two, 0));
    }

    // test the Strike::Score behavior in a second scenario, striking the enemy "edge" of the input space
    #[test]
    fn test_strike_score_edge() {
        let mut app = app();
        let size = 7;

        app.world_mut().trigger(SpawnGame::new(size));
        // update to let spawns / etc flush
        app.update();

        let (player_one, player_two) = find_players(app.world_mut());

        let first_six_letters = ALPHABET[0..6].iter().cloned().collect();
        set_word(app.world_mut(), player_one, first_six_letters);

        // nothing should happen here, but update to prevent influencing tests of future mutations
        app.update();

        assert_word_sizes(app.world(), (player_one, 6), (player_two, 0));
        assert_scores(app.world(), (player_one, 0), (player_two, 0));

        app.world_mut().send_event::<ActionEvent>(
            Action::Append(ALPHABET[6]).made_by(player_one, PlayerSide::Left),
        );
        // update twice to process the event through replicon
        app.update();
        app.update();

        assert_word_sizes(app.world(), (player_one, 0), (player_two, 0));
        assert_scores(app.world(), (player_one, 1), (player_two, 0));
    }

    // test a Strike::Score interaction where each letter is fired via an action one at a time
    #[test]
    fn test_strike_full_interaction() {
        let mut app = app();
        let size = 7;

        app.world_mut().trigger(SpawnGame::new(size));
        // update to let spawns / etc flush
        app.update();

        let (player_one, player_two) = find_players(app.world_mut());

        // add the first 3 letters of the word to left and then right each letter
        for (index, letter) in ALPHABET[0..3].iter().enumerate() {
            // first left
            assert_word_sizes(app.world(), (player_one, index), (player_two, index));
            assert_scores(app.world(), (player_one, 0), (player_two, 0));

            app.world_mut().send_event::<ActionEvent>(
                Action::Append(*letter).made_by(player_one, PlayerSide::Left),
            );
            app.update();
            app.update();

            // then right
            assert_word_sizes(app.world(), (player_one, index + 1), (player_two, index));
            assert_scores(app.world(), (player_one, 0), (player_two, 0));

            app.world_mut().send_event::<ActionEvent>(
                Action::Append(*letter).made_by(player_two, PlayerSide::Right),
            );
            app.update();
            app.update();
        }

        assert_word_sizes(app.world(), (player_one, 3), (player_two, 3));
        assert_scores(app.world(), (player_one, 0), (player_two, 0));

        app.world_mut().send_event::<ActionEvent>(
            Action::Append(ALPHABET[3]).made_by(player_two, PlayerSide::Right),
        );
        // update twice to process the event through replicon
        app.update();
        app.update();

        assert_word_sizes(app.world(), (player_one, 0), (player_two, 0));
        assert_scores(app.world(), (player_one, 0), (player_two, 1));
    }

    // test the Strike::Parry behavior
    #[test]
    fn test_strike_parry() {
        let mut app = app();
        let size = 7;

        app.world_mut().trigger(SpawnGame::new(size));
        // update to let spawns / etc flush
        app.update();

        let (player_one, player_two) = find_players(app.world_mut());

        let first_three_letters: Vec<Letter> = ALPHABET[0..3].iter().cloned().collect();
        set_word(app.world_mut(), player_one, first_three_letters.clone());
        set_word(app.world_mut(), player_two, first_three_letters);

        // nothing should happen here, but update to prevent influencing tests of future mutations
        app.update();

        assert_word_sizes(app.world(), (player_one, 3), (player_two, 3));
        assert_scores(app.world(), (player_one, 0), (player_two, 0));

        // send the 3rd letter again to create a tie, resulting in a Strike::Parry
        app.world_mut().send_event::<ActionEvent>(
            Action::Append(ALPHABET[2]).made_by(player_one, PlayerSide::Left),
        );
        // update twice to process the event through replicon
        app.update();
        app.update();

        assert_word_sizes(app.world(), (player_one, 0), (player_two, 0));
        assert_scores(app.world(), (player_one, 0), (player_two, 0));

        // now try the same for the other player

        let first_three_letters: Vec<Letter> = ALPHABET[0..3].iter().cloned().collect();
        set_word(app.world_mut(), player_one, first_three_letters.clone());
        set_word(app.world_mut(), player_two, first_three_letters);
        app.update();

        assert_word_sizes(app.world(), (player_one, 3), (player_two, 3));
        assert_scores(app.world(), (player_one, 0), (player_two, 0));
        app.world_mut().send_event::<ActionEvent>(
            Action::Append(ALPHABET[2]).made_by(player_two, PlayerSide::Right),
        );
        // update twice to process the event through replicon
        app.update();
        app.update();

        assert_word_sizes(app.world(), (player_one, 0), (player_two, 0));
        assert_scores(app.world(), (player_one, 0), (player_two, 0));
    }

    // test the Strike::OverRange behavior
    #[test]
    fn test_strike_over_range() {
        let mut app = app();
        let size = 7;

        app.world_mut().trigger(SpawnGame::new(size));
        // update to let spawns / etc flush
        app.update();

        let (player_one, player_two) = find_players(app.world_mut());

        let first_three_letters: Vec<Letter> = ALPHABET[0..3].iter().cloned().collect();
        set_word(app.world_mut(), player_one, first_three_letters.clone());
        set_word(app.world_mut(), player_two, first_three_letters);

        // nothing should happen here, but update to prevent influencing tests of future mutations
        app.update();

        assert_word_sizes(app.world(), (player_one, 3), (player_two, 3));
        assert_scores(app.world(), (player_one, 0), (player_two, 0));

        // two inputs at the same time!
        app.world_mut().send_event::<ActionEvent>(
            Action::Append(ALPHABET[3]).made_by(player_one, PlayerSide::Left),
        );
        app.world_mut().send_event::<ActionEvent>(
            Action::Append(ALPHABET[3]).made_by(player_two, PlayerSide::Right),
        );
        // update twice to process the event through replicon
        app.update();
        app.update();

        assert_word_sizes(app.world(), (player_one, 0), (player_two, 0));
        assert_scores(app.world(), (player_one, 0), (player_two, 0));
    }
}
