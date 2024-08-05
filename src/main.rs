// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::{App, AppExit, Commands, Startup};

use game::SpawnGame;
use wordfight::WordFightPlugins;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(WordFightPlugins);
    app.add_systems(Startup, spawn_game);
    app.run()
}

fn spawn_game(mut commands: Commands) {
    commands.trigger(SpawnGame::new(7));
}
