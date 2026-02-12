//

use bevy::prelude::*;

use dmak26::msg;
use dmak26::shared;
use dmak26::srv;

use core::time::Duration;

use bevy::diagnostic::DiagnosticsPlugin;

use bevy::state::app::StatesPlugin;

use bevy::log::{Level, LogPlugin};

use lightyear::prelude::server::*;

fn main() {
    println!("this is a server!");

    let mut app = App::new();

    app.add_plugins(MinimalPlugins);
    app.add_plugins(LogPlugin {
        level: Level::INFO,
        filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,bevy_time=warn,naga=warn,bevy_enhanced_input::action::fns=error".to_string(),
        ..default()
    });
    app.add_plugins(StatesPlugin);
    app.add_plugins(DiagnosticsPlugin);

    let tick_duration = Duration::from_secs_f64(1.0 / shared::FIXED_TIMESTEP_HZ);

    app.add_plugins((lightyear::prelude::server::ServerPlugins { tick_duration },));

    app.add_plugins(msg::ProtocolPlugin);

    let _server = app
        .world_mut()
        .spawn(srv::ExampleServer) // {
            //conditioner: None,
            // transport: ServerTransports::Udp {
            //     local_port: SERVER_PORT,
            // },
            // transport: ServerTransports::WebSocket {
            //     local_port: SERVER_PORT,
            // },
            /*
            transport: ServerTransports::WebTransport {
                local_port: SERVER_PORT,
                certificate: WebTransportCertificateSettings::FromFile {
                    cert: "../../certificates/cert.pem".to_string(),
                    key: "../../certificates/key.pem".to_string(),
                },
            },
            */
            // #[cfg(feature = "steam")]
            // transport: ServerTransports::Steam {
            //     local_port: SERVER_PORT,
            // },
            //shared: SHARED_SETTINGS,
        //})
        .id();

    app.add_systems(Startup, start);

    app.add_plugins(shared::HelloPlugin)
        .add_plugins(shared::CounterPlugin)
        .run();
}

fn start(mut commands: Commands, server: Single<Entity, With<Server>>) {
    commands.trigger(Start {
        entity: server.into_inner(),
    });
}
