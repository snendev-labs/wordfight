// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::{App, AppExit};

use wordfight::WordFightPlugins;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(WordFightPlugins);
    app.run()
}
