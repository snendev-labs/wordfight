use bevy::{
    log::info,
    prelude::{
        App, Commands, Entity, EventReader, IntoSystemConfigs, Name, Plugin, Query, Res, ResMut,
        Startup, Update, With, Without,
    },
};

use bevy_replicon::prelude::{ConnectedClients, Replicated, RepliconChannels, ServerEvent};
use bevy_replicon_renet2::{
    renet2::{ConnectionConfig, RenetServer},
    RenetChannelsExt, RepliconRenetServerPlugin,
};

use game::{Client, InGame, SpawnGame};

mod transport;
use transport::*;

pub struct ServerPlugin {
    pub port: String,
    pub wt_tokens_port: String,
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconRenetServerPlugin);

        app.add_plugins(ServerTransportPlugin {
            port: self.port.clone(),
            wt_tokens_port: self.wt_tokens_port.clone(),
        });
        app.add_systems(Startup, Self::start_server).add_systems(
            Update,
            (
                Self::handle_connections,
                Self::matchmake,
                Self::handle_visibility,
            )
                .chain(),
        );
    }
}

impl ServerPlugin {
    fn start_server(mut commands: Commands, replicon_channels: Res<RepliconChannels>) {
        let server_channels_config = replicon_channels.get_server_configs();
        let client_channels_config = replicon_channels.get_client_configs();

        info!(
            "Starting server! Server channels: {} | Client channels: {}",
            server_channels_config.len(),
            client_channels_config.len()
        );
        let server = RenetServer::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            ..Default::default()
        });
        commands.insert_resource(server);
    }

    fn matchmake(mut commands: Commands, clients: Query<Entity, (With<Client>, Without<InGame>)>) {
        for [client1, client2] in clients
            .iter()
            .collect::<Vec<_>>()
            .chunks_exact(2)
            .map(|chunk| [chunk[0], chunk[1]])
        {
            info!("Found match: {client1} + {client2}");
            commands.trigger(SpawnGame::new(7, client1, client2));
        }
    }

    fn handle_connections(
        mut commands: Commands,
        mut server_events: EventReader<ServerEvent>,
        clients: Query<(Entity, &Client)>,
    ) {
        for event in server_events.read() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    info!("Player {} connected.", client_id.get());
                    // Spawn new player entity
                    commands.spawn((
                        Replicated,
                        Name::new(format!("Player {}", client_id.get())),
                        Client::from(*client_id),
                    ));
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    if let Some((player_entity, _)) =
                        clients.iter().find(|(_, id)| ***id == *client_id)
                    {
                        info!("Player disconnected: {}", reason);
                        commands.entity(player_entity).despawn();
                    }
                }
            }
        }
    }

    pub fn handle_visibility(
        players: Query<(Entity, &Client, Option<&InGame>)>,
        game_entities: Query<(Entity, &InGame), Without<Client>>,
        mut connected_clients: ResMut<ConnectedClients>,
    ) {
        // let players have visibility over all entities present in the same game
        for (entity, player, player_game) in players
            .iter()
            .filter_map(|(entity, player, in_game)| in_game.map(|game| (entity, player, game)))
        {
            let client = connected_clients.client_mut(**player);
            let visibility = client.visibility_mut();
            // player can see themselves
            visibility.set_visibility(entity, true);
            // and the game instance
            // TODO: turning this off when switching / ending games?
            visibility.set_visibility(**player_game, true);

            // player can see other game entities
            for (entity, in_game) in game_entities.iter() {
                if **in_game == **player_game {
                    visibility.set_visibility(entity, true);
                } else {
                    visibility.set_visibility(entity, false);
                }
            }
        }

        // players also need to be able to see each other when either both in lobby, or both in the same game
        for [(entity1, player1, in_game1), (entity2, player2, in_game2)] in
            players.iter_combinations()
        {
            let visible = match (in_game1, in_game2) {
                (None, None) => true,
                (None, Some(_)) | (Some(_), None) => false,
                (Some(game1), Some(game2)) => {
                    if **game1 == **game2 {
                        true
                    } else {
                        false
                    }
                }
            };
            let client1 = connected_clients.client_mut(**player1);
            let visibility1 = client1.visibility_mut();
            visibility1.set_visibility(entity2, visible);

            let client2 = connected_clients.client_mut(**player2);
            let visibility2 = client2.visibility_mut();
            visibility2.set_visibility(entity1, visible);
        }
    }
}
