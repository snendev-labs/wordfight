use bevy::app::{PluginGroup, PluginGroupBuilder};

pub use game::*;
#[cfg(feature = "server")]
pub use server::*;
#[cfg(feature = "ui")]
pub use ui::*;

pub struct WordFightPlugins;

impl PluginGroup for WordFightPlugins {
    fn build(self) -> PluginGroupBuilder {
        let plugins = PluginGroupBuilder::start::<Self>();
        let plugins = plugins.add(WordFightGamePlugin);
        #[cfg(feature = "server")]
        let plugins = plugins.add(WordFightServerPlugin);
        #[cfg(feature = "ui")]
        let plugins = plugins.add(WordFightUiPlugin);
        plugins
    }
}
