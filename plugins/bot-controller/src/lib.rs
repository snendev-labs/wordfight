use bevy::prelude::*;

pub struct BotControllerPlugin;

impl Plugin for BotControllerPlugin {
    fn build(&self, _app: &mut App) {
        //
    }
}

impl BotControllerPlugin {
    //
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(SystemSet)]
pub struct BotControllerSystems;

#[derive(Debug)]
#[derive(Component, Reflect)]
pub struct BotController;
