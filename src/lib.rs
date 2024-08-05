use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    DefaultPlugins,
};

pub use game::*;
#[cfg(feature = "server")]
pub use server::*;
#[cfg(feature = "ui")]
pub use ui::*;

pub struct WordFightPlugins;

impl PluginGroup for WordFightPlugins {
    fn build(self) -> PluginGroupBuilder {
        let plugins = PluginGroupBuilder::start::<Self>();
        let default_plugins = DefaultPlugins;
        #[cfg(feature = "ui")]
        let default_plugins = default_plugins.set(bevy::window::WindowPlugin {
            primary_window: bevy::window::Window {
                title: "WordFight".to_string(),
                canvas: Some("#bevy".to_string()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });
        let plugins = plugins.add_group(default_plugins);
        let plugins = plugins.add(WordFightGamePlugin);
        #[cfg(feature = "server")]
        let plugins = plugins.add(WordFightServerPlugin);
        #[cfg(feature = "ui")]
        let plugins = plugins.add(WordFightUiPlugin);
        #[cfg(feature = "dev")]
        let plugins = plugins.add(bevy_inspector_egui::quick::WorldInspectorPlugin::default());
        plugins
    }
}
