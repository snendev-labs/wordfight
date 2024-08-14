use std::net::SocketAddr;
use url::Url;

use bevy::prelude::{App, Plugin};
use renet2::transport::WebServerDestination;

use game::PROTOCOL_ID;

pub struct ClientTransportPlugin {
    server_address: WebServerDestination,
    server_token: String,
}

impl ClientTransportPlugin {
    pub fn new(host: &str, port: &str, server_token: &str) -> Self {
        Self::ip(host, port, server_token)
            // .or_else(|| Self::url(host, port, server_token))
            .unwrap()
    }

    fn url(host: &str, port: &str, server_token: &str) -> Option<Self> {
        format!("{host}:{port}")
            .parse::<Url>()
            .map(|url| Self {
                server_address: WebServerDestination::Url(url),
                server_token: server_token.to_string(),
            })
            .ok()
    }

    fn ip(ip: &str, port: &str, server_token: &str) -> Option<Self> {
        format!("{ip}:{port}")
            .parse::<SocketAddr>()
            .map(|addr| Self {
                server_address: WebServerDestination::Addr(addr),
                server_token: server_token.to_string(),
            })
            .ok()
    }
}

#[cfg(target_family = "wasm")]
impl Plugin for ClientTransportPlugin {
    fn build(&self, app: &mut App) {
        use renet2::transport::{ClientAuthentication, NetcodeClientTransport};
        use wasm_timer::SystemTime;

        let server_addr: SocketAddr = self.server_address.clone().into();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            socket_id: 0,
            server_addr,
            user_data: None,
        };

        // TODO: to support this at this layer we would need to pass the client socket in from above
        // #[cfg(feature = "memory_transport")]
        // let socket = renet2::transport::MemorySocketClient::new(client_id as u16, client_memory_socket).unwrap();
        use base64::Engine;
        use renet2::transport::{ServerCertHash, WebTransportClientConfig};

        let hash = base64::engine::general_purpose::STANDARD
            .decode(self.server_token.clone())
            .unwrap();
        let config = WebTransportClientConfig::new_with_certs(
            server_addr,
            Vec::from([ServerCertHash::try_from(hash).unwrap()]),
        );
        let socket = renet2::transport::WebTransportClient::new(config);
        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
        app.insert_resource(transport);
    }
}
