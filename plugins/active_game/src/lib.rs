use bevy::prelude::*;

use game::{Arena, Game, GamePlayers, Score, Word};

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
    fn set_active_game(mut commands: Commands, spawned_games: Query<Entity, With<Game>>) {
        if let Some(game) = spawned_games.iter().next() {
            info!("Setting active game: {game}");
            commands.insert_resource(ActiveGame(game));
        }
    }

    fn trigger_game_update(
        mut commands: Commands,
        mut events: EventWriter<ActiveGameUpdate>,
        active_game: Res<ActiveGame>,
        games: Query<(&GamePlayers, &Arena)>,
        words: Query<(&Word, &Score)>,
        updated_words: Query<(), Changed<Word>>,
        updated_scores: Query<(), Changed<Score>>,
    ) {
        let Some((game, players, arena)) = games
            .get(active_game.0)
            .ok()
            .map(|(players, arena)| (active_game, players, arena))
            .filter(|(game, players, _)| {
                game.is_changed()
                    || updated_words.contains(players.left)
                    || updated_words.contains(players.right)
                    || updated_scores.contains(players.left)
                    || updated_scores.contains(players.right)
            })
        else {
            return;
        };

        let (left_word, left_score) = words
            .get(players.left)
            .map(|(word, score)| (word.clone(), *score))
            .expect("PlayerSide::Left to have a Word");
        let (right_word, right_score) = words
            .get(players.right)
            .map(|(word, score)| (word.clone(), *score))
            .expect("PlayerSide::Right should have a Word");
        let event = ActiveGameUpdate {
            game: game.0,
            arena_size: arena.size(),
            player_left: players.left,
            left_word,
            left_score,
            player_right: players.right,
            right_word,
            right_score,
        };
        info!("Game update triggered: {event:?}");
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
    pub left_score: Score,
    pub player_right: Entity,
    pub right_word: Word,
    pub right_score: Score,
}
