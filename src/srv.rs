//! The server side of the example.
//! It is possible (and recommended) to run the server in headless mode (without any rendering plugins).
//!
//! The server will:
//! - spawn a new player entity for each client that connects
//! - read inputs from the clients and move the player entities accordingly
//!
//! Lightyear will handle the replication of entities automatically if you add a `Replicate` component to them.

use crate::msg::*;
use crate::shared;
use shared::SEND_INTERVAL;

use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

use lightyear::netcode::NetcodeServer;
//use lightyear::netcode::PRIVATE_KEY_BYTES;
use lightyear::connection::client::Connected;
use lightyear::prelude::input::native::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

use core::net::{Ipv4Addr, SocketAddr};

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        // the physics/FixedUpdates systems that consume inputs should be run in this set.
        app.add_systems(FixedUpdate, movement);
        app.add_observer(handle_new_client);
        app.add_observer(handle_connected);
    }
}

/// When a new client tries to connect to a server, an entity is created for it with the `LinkOf` component.
/// This entity represents the link between the server and that client.
///
/// You can add additional components to update the link. In this case we will add a `ReplicationSender` that
/// will enable us to replicate local entities to that client.
pub(crate) fn handle_new_client(trigger: On<Add, LinkOf>, mut commands: Commands) {
    commands.entity(trigger.entity).insert((
        ReplicationSender::new(SEND_INTERVAL, SendUpdatesMode::SinceLastAck, false),
        Name::from("Client"),
    ));
}

/// If the new client connects to the server, we want to spawn a new player entity for it.
///
/// We have to react specifically on `Connected` because there is no guarantee that the connection request we
/// received was valid. The server could reject the connection attempt for many reasons (server is full, packet is invalid,
/// DDoS attempt, etc.). We want to start the replication only when the client is confirmed as connected.
pub(crate) fn handle_connected(
    trigger: On<Add, Connected>,
    query: Query<&RemoteId, With<ClientOf>>,
    mut commands: Commands,
) {
    let Ok(client_id) = query.get(trigger.entity) else {
        return;
    };
    let client_id = client_id.0;
    let entity = commands
        .spawn((
            PlayerBundle::new(client_id, Vec2::ZERO),
            // we replicate the Player entity to all clients that are connected to this server
            Replicate::to_clients(NetworkTarget::All),
            PredictionTarget::to_clients(NetworkTarget::Single(client_id)),
            InterpolationTarget::to_clients(NetworkTarget::AllExceptSingle(client_id)),
            ControlledBy {
                owner: trigger.entity,
                lifetime: Default::default(),
            },
        ))
        .id();
    info!(
        "Create player entity {:?} for client {:?}",
        entity, client_id
    );
}

/// Read client inputs and move players in server therefore giving a basis for other clients
fn movement(
    timeline: Res<LocalTimeline>,
    mut position_query: Query<
        (&mut PlayerPosition, &ActionState<Inputs>),
        // if we run in host-server mode, we don't want to apply this system to the local client's entities
        // because they are already moved by the client plugin
        Without<Predicted>,
    >,
) {
    let tick = timeline.tick();
    for (position, inputs) in position_query.iter_mut() {
        trace!(?tick, ?position, ?inputs, "server");
        shared::shared_movement_behaviour(position, inputs);
    }
}

#[derive(Component, Debug)]
#[component(on_add = ExampleServer::on_add)]
pub struct ExampleServer;

impl ExampleServer {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let entity = context.entity;
        world.commands().queue(move |world: &mut World| -> Result {
            let mut entity_mut = world.entity_mut(entity);
            let _settings = entity_mut.take::<ExampleServer>().unwrap();
            entity_mut.insert((Name::from("Server"),));

            let add_netcode = |entity_mut: &mut EntityWorldMut| {
                let private_key = shared::SHARED_SETTINGS.private_key;
                entity_mut.insert(NetcodeServer::new(NetcodeConfig {
                    protocol_id: shared::SHARED_SETTINGS.protocol_id,
                    private_key,
                    ..Default::default()
                }));
            };

            let local_port = shared::SERVER_PORT;
            add_netcode(&mut entity_mut);
            let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), local_port);
            entity_mut.insert((
                LocalAddr(server_addr),
                ServerUdpIo::default(),
                /*
                WebTransportServerIo {
                    certificate: (&certificate).into(),
                },
                // */
            ));
            /*
            ServerTransports::WebTransport {
                local_port,
                certificate,
            } => {
                add_netcode(&mut entity_mut);
                let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), local_port);
                entity_mut.insert((
                    LocalAddr(server_addr),
                    WebTransportServerIo {
                        certificate: (&certificate).into(),
                    },
                ));
            }
            ServerTransports::WebSocket { local_port } => {
                add_netcode(&mut entity_mut);
                let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), local_port);
                let sans = vec![
                    "localhost".to_string(),
                    "127.0.0.1".to_string(),
                    "::1".to_string(),
                ];
                let config = ServerConfig::builder()
                    .with_bind_address(server_addr)
                    .with_identity(
                        lightyear::websocket::server::Identity::self_signed(sans).unwrap(),
                    );
                entity_mut.insert((LocalAddr(server_addr), WebSocketServerIo { config }));
            }
            #[cfg(feature = "steam")]
            ServerTransports::Steam { local_port } => {
                let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), local_port);
                entity_mut.insert(SteamServerIo {
                    target: ListenTarget::Addr(server_addr),
                    config: SessionConfig::default(),
                });
            }
            */
            Ok(())
        });
    }
}
