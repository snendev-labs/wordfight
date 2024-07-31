use bevy::{prelude::*, DefaultPlugins};

use wordfight::WordFightPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(WordFightPlugin);
    app.run();
}
