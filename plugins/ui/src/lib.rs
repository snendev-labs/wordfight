use bevy::{color::palettes, prelude::*};
use bevy_mod_try_system::*;
use sickle_ui::prelude::*;

use game::{Arena, Game, GamePlayers, Letter, Word};

pub struct WordFightUiPlugin;

impl Plugin for WordFightUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_camera);
        app.add_systems(
            Update,
            (
                Self::set_active_game.run_if(not(resource_exists::<ActiveGame>)),
                Self::spawn_game_ui.log_err().run_if(
                    resource_exists_and_changed::<ActiveGame>.or_else(Self::active_game_updated),
                ),
            ),
        );
    }
}

impl WordFightUiPlugin {
    fn spawn_camera(mut commands: Commands) {
        commands.spawn(Camera2dBundle::default());
    }

    fn set_active_game(mut commands: Commands, spawned_games: Query<Entity, Added<Game>>) {
        for game in spawned_games.iter() {
            commands.insert_resource(ActiveGame(game));
        }
    }

    fn spawn_game_ui(
        mut commands: Commands,
        active_game: Res<ActiveGame>,
        games: Query<(&Arena, &GamePlayers), With<Game>>,
        words: Query<&Word>,
    ) -> anyhow::Result<()> {
        let (arena, players) = games.get(active_game.0)?;
        let size = arena.size();
        let [left_word, right_word] = words.get_many([players.left, players.right])?;
        let mut game_letters: Vec<LetterSlot> = vec![LetterSlot::Empty; size];
        for (index, letter) in left_word.iter().enumerate() {
            game_letters.insert(index, LetterSlot::Letter(*letter));
        }
        for (letter_index, letter) in right_word.iter().enumerate() {
            let index = size - 1 - letter_index;
            let new_letter = match game_letters.get(index) {
                Some(LetterSlot::Letter(_) | LetterSlot::OverWritten) => LetterSlot::OverWritten,
                _ => LetterSlot::Letter(*letter),
            };
            game_letters.insert(index, new_letter);
        }

        commands
            .ui_builder(UiRoot)
            .row(|row| {
                for letter in game_letters {
                    letter.ui(row);
                }
            })
            .insert(GameUi)
            .style()
            .width(Val::Percent(100.))
            .height(Val::Percent(100.))
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .padding(UiRect::new(
                Val::Percent(20.),
                Val::Percent(20.),
                Val::Auto,
                Val::Auto,
            ))
            .column_gap(Val::Px(12.))
            .background_color(Color::Srgba(palettes::tailwind::CYAN_300));

        Ok(())
    }

    fn active_game_updated(
        active_game: Option<Res<ActiveGame>>,
        games: Query<&GamePlayers>,
        updated_words: Query<(), Changed<Word>>,
    ) -> bool {
        active_game
            .and_then(|game| games.get(game.0).ok())
            .is_some_and(|players| {
                updated_words.contains(players.left) || updated_words.contains(players.right)
            })
    }
}

#[derive(Clone, Copy)]
enum LetterSlot {
    Empty,
    OverWritten,
    Letter(Letter),
}

impl LetterSlot {
    fn ui(&self, ui: &mut UiBuilder<Entity>) {
        ui.row(|ui| match self {
            LetterSlot::Letter(letter) => Self::letter_ui(ui, *letter),
            LetterSlot::Empty => Self::empty_ui(ui),
            LetterSlot::OverWritten => Self::overwritten_ui(ui),
        })
        .style()
        .font_size(48.)
        .height(Val::Px(80.))
        .width(Val::Px(80.))
        .border(UiRect::px(2., 2., 0., 4.))
        .border_color(Color::srgba(0.2, 0.2, 0.2, 0.9))
        .background_color(Color::srgba(0., 0., 0., 0.05));
    }

    fn empty_ui(_ui: &mut UiBuilder<Entity>) {}

    fn overwritten_ui(ui: &mut UiBuilder<Entity>) {
        ui.label(LabelConfig::from("@"));
    }

    fn letter_ui(ui: &mut UiBuilder<Entity>, letter: Letter) {
        ui.label(LabelConfig::from(letter.to_string()));
    }
}

#[derive(Debug)]
#[derive(Resource, Reflect)]
pub struct ActiveGame(Entity);

#[derive(Debug)]
#[derive(Component, Reflect)]
pub struct GameUi;
