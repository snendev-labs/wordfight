use bevy::{
    ecs::world::Command,
    log::info,
    prelude::{App, Commands, IntoSystem, Plugin, Startup, Update, World},
};

use bevy_replicon::core::common_conditions as network_conditions;
use bevy_replicon::prelude::RepliconChannels;
use bevy_replicon_renet2::{
    renet2::{ConnectionConfig, RenetClient},
    RenetChannelsExt, RepliconRenetClientPlugin,
};

mod transport;

pub struct ClientPlugin {
    pub server_origin: String,
    pub server_port: String,
    pub server_token: String,
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconRenetClientPlugin);

        #[cfg(target_family = "wasm")]
        app.add_plugins(transport::ClientTransportPlugin::new(
            &self.server_origin,
            &self.server_port,
            &self.server_token,
        ));
        app.add_systems(Startup, |mut commands: Commands| {
            commands.add(ClientCommand::Connect);
        });
        app.add_systems(
            Update,
            network_conditions::client_just_connected.map(|just_connected| {
                if just_connected {
                    info!("Connected!");
                }
            }),
        );
    }
}

pub enum ClientCommand {
    Connect,
    Disconnect,
}

impl Command for ClientCommand {
    fn apply(self, world: &mut World) {
        match self {
            ClientCommand::Connect => {
                connect_to_server(world);
            }
            ClientCommand::Disconnect => {
                world.remove_resource::<RenetClient>();
            }
        }
    }
}

// TODO: turn this into a system once bevy_renet2 uses the run condition here
// https://github.com/UkoeHB/renet2/blob/main/bevy_renet2/src/lib.rs#L62
fn connect_to_server(world: &mut World) {
    let replicon_channels = world
        .get_resource::<RepliconChannels>()
        .expect("replicon plugins to be added before transport plugins");
    let server_channels_config = replicon_channels.get_server_configs();
    let client_channels_config = replicon_channels.get_client_configs();
    let client = RenetClient::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });
    world.insert_resource(client);
}
