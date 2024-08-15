use bevy::{app::PluginGroupBuilder, prelude::*, DefaultPlugins};

pub use active_game::*;
pub use game::*;

pub struct WordFightPlugins;

impl PluginGroup for WordFightPlugins {
    fn build(self) -> PluginGroupBuilder {
        let plugins = PluginGroupBuilder::start::<Self>();
        let plugins = plugins.add_group(DefaultPlugins);
        let plugins = plugins.add(WordFightGamePlugin).add(ActiveGamePlugin);
        #[cfg(feature = "dev")]
        let plugins = plugins.add(StartupPlugin);
        #[cfg(feature = "dev")]
        let plugins = plugins.add(bevy_inspector_egui::quick::WorldInspectorPlugin::default());
        plugins
    }
}

#[cfg(feature = "dev")]
pub struct StartupPlugin;

#[cfg(feature = "dev")]
impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_game);
    }
}

#[cfg(feature = "dev")]
impl StartupPlugin {
    fn spawn_game(mut commands: Commands) {
        let client1 = commands.spawn(Client::from(ClientId::SERVER).bundle()).id();
        let client2 = commands.spawn(Client::from(ClientId::SERVER).bundle()).id();
        commands.trigger(SpawnGame::new(7, client1, client2));
    }
}
