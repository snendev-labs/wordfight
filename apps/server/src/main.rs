use bevy::{
    app::ScheduleRunnerPlugin,
    log::{Level, LogPlugin},
    prelude::*,
};

use server::ServerPlugin;
use wordfight::{ActiveGamePlugin, WordFightPlugins};

fn main() {
    App::default()
        .add_plugins((
            ScheduleRunnerPlugin::run_loop(
                // need some wait duration so that async tasks are not entirely outcompeted by the main loop
                std::time::Duration::from_millis(10),
            ),
            WordFightPlugins
                .build()
                .set(LogPlugin {
                    filter: "wgpu=error,naga=warn,h3=error".to_string(),
                    level: Level::INFO,
                    ..Default::default()
                })
                .disable::<ActiveGamePlugin>(),
        ))
        .add_plugins(ServerPlugin {
            port: option_env!("SERVER_PORT").unwrap_or("7636").to_string(),
            wt_tokens_port: option_env!("SERVER_TOKENS_PORT")
                .unwrap_or("7637")
                .to_string(),
            // native_host: option_env!("SERVER_IP")
            //     .unwrap_or("127.0.0.1")
            //     .to_string(),
            // native_port: option_env!("SERVER_IP")
            //     .unwrap_or("127.0.0.1")
            //     .to_string(),
        })
        .run();
}
