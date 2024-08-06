use bevy::prelude::*;

use game::{Arena, Game, GamePlayers, Word};

pub struct ActiveGamePlugin;

impl Plugin for ActiveGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActiveGameUpdate>()
            .add_systems(
                Update,
                Self::set_active_game.run_if(not(resource_exists::<ActiveGame>)),
            )
            .add_systems(
                Update,
                Self::trigger_game_update.run_if(resource_exists::<ActiveGame>),
            );
    }
}

impl ActiveGamePlugin {
    fn set_active_game(mut commands: Commands, spawned_games: Query<Entity, Added<Game>>) {
        for game in spawned_games.iter() {
            commands.insert_resource(ActiveGame(game));
        }
    }

    fn trigger_game_update(
        mut commands: Commands,
        mut events: EventWriter<ActiveGameUpdate>,
        active_game: Res<ActiveGame>,
        games: Query<(&GamePlayers, &Arena)>,
        words: Query<&Word>,
        updated_words: Query<(), Changed<Word>>,
    ) {
        let Some((game, players, arena)) = games
            .get(active_game.0)
            .ok()
            .map(|(players, arena)| (active_game, players, arena))
            .filter(|(game, players, _)| {
                game.is_changed()
                    || updated_words.contains(players.left)
                    || updated_words.contains(players.right)
            })
        else {
            return;
        };

        let left_word = words
            .get(players.left)
            .cloned()
            .expect("PlayerSide::Left to have a Word");
        let right_word = words
            .get(players.right)
            .cloned()
            .expect("PlayerSide::Right should have a Word");
        let event = ActiveGameUpdate {
            game: game.0,
            arena_size: arena.size(),
            player_left: players.left,
            left_word,
            player_right: players.right,
            right_word,
        };
        commands.trigger(event.clone());
        events.send(event);
    }
}

#[derive(Debug)]
#[derive(Deref, DerefMut, Resource, Reflect)]
pub struct ActiveGame(Entity);

// Can be read from an EventReader or observed as needed
#[derive(Clone, Debug)]
#[derive(Event)]
pub struct ActiveGameUpdate {
    pub game: Entity,
    pub arena_size: usize,
    pub player_left: Entity,
    pub left_word: Word,
    pub player_right: Entity,
    pub right_word: Word,
}
