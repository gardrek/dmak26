//

use core::net::Ipv4Addr;
use core::net::SocketAddr;

use core::time::Duration;

use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

// #[cfg(not(target_family = "wasm"))]
use bevy::asset::uuid::Uuid;

use lightyear::prelude::client::input::*;
use lightyear::prelude::client::*;
use lightyear::prelude::input::native::*;
use lightyear::prelude::*;

use dmak26::render;
use dmak26::shared;
use dmak26::shared::*;

use dmak26::msg::*;

//~ #[cfg(not(target_family = "wasm"))]
fn gen_random_id() -> u64 {
    let uuid = Uuid::new_v4();

    let (a, b) = uuid.as_u64_pair();

    a ^ b
}

fn main() {
    println!("this is a client!");
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(shared::CounterPlugin)
        .add_plugins(render::CounterRenderPlugin);

    //.add_plugins(shared::AutosavePlugin)

    app.add_plugins(ProtocolPlugin);

    let tick_duration = Duration::from_secs_f64(1.0 / shared::FIXED_TIMESTEP_HZ);

    app.add_plugins(ClientPlugins { tick_duration });

    spawn_connections(&mut app);

    app.add_plugins(ClientPlugin);

    app.run();
}

/// Event that examples can trigger to spawn a client.
#[derive(Component, Clone, Debug)]
#[component(on_add = ClientSettingsThing::on_add)]
pub struct ClientSettingsThing {
    client_id: u64,
}

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Startup, Self::setup);

        app.add_systems(
            FixedPreUpdate,
            // Inputs have to be buffered in the WriteClientInputs set
            buffer_input.in_set(InputSystems::WriteClientInputs),
        );
        app.add_systems(FixedUpdate, player_movement);

        //app.add_systems(Update, receive_message1);
        app.add_observer(handle_predicted_spawn);
        app.add_observer(handle_interpolated_spawn);
    }
}

impl ClientSettingsThing {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let entity = context.entity;
        //*
        world.commands().queue(move |world: &mut World| -> Result {
            let mut entity_mut = world.entity_mut(entity);
            let settings = entity_mut.take::<ClientSettingsThing>().unwrap();
            let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), shared::CLIENT_PORT);
            entity_mut.insert((
                Client::default(),
                Link::new(None),
                LocalAddr(client_addr),
                PeerAddr(shared::SERVER_ADDR),
                ReplicationReceiver::default(),
                PredictionManager::default(),
                Name::from("Client"),
            ));

            let add_netcode = |entity_mut: &mut EntityWorldMut| -> Result {
                // use dummy zeroed key explicitly here.
                let auth = Authentication::Manual {
                    server_addr: shared::SERVER_ADDR,
                    client_id: settings.client_id,
                    private_key: SHARED_SETTINGS.private_key,
                    protocol_id: SHARED_SETTINGS.protocol_id,
                };
                let netcode_config = NetcodeConfig {
                    // Make sure that the server times out clients when their connection is closed
                    client_timeout_secs: 3,
                    token_expire_secs: -1,
                    ..default()
                };
                entity_mut.insert(NetcodeClient::new(auth, netcode_config)?);
                Ok(())
            };

            add_netcode(&mut entity_mut)?;
            #[cfg(not(target_family = "wasm"))]
            entity_mut.insert(UdpIo::default());
            #[cfg(target_family = "wasm")]
            {
                let certificate_digest =
                    { include_str!("../../certificate_digest.txt").to_string() };
                entity_mut.insert(WebTransportClientIo { certificate_digest });
            }

            Ok(())
        });
        // */
    }
}

/// System that reads from peripherals and adds inputs to the buffer
/// This system must be run in the `InputSystemSet::BufferInputs` set in the `FixedPreUpdate` schedule
/// to work correctly.
///
/// I would also advise to use the `leafwing` feature to use the `LeafwingInputPlugin` instead of the
/// `InputPlugin`, which contains more features.
pub(crate) fn buffer_input(
    mut query: Query<&mut ActionState<Inputs>, With<InputMarker<Inputs>>>,
    keypress: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut action_state) = query.single_mut() {
        let mut up = 0.0;
        let mut down = 0.0;
        let mut left = 0.0;
        let mut right = 0.0;
        let mut fire = false;

        if keypress.pressed(KeyCode::KeyW) || keypress.pressed(KeyCode::ArrowUp) {
            up = 1.0;
        }
        if keypress.pressed(KeyCode::KeyS) || keypress.pressed(KeyCode::ArrowDown) {
            down = 1.0;
        }
        if keypress.pressed(KeyCode::KeyA) || keypress.pressed(KeyCode::ArrowLeft) {
            left = 1.0;
        }
        if keypress.pressed(KeyCode::KeyD) || keypress.pressed(KeyCode::ArrowRight) {
            right = 1.0;
        }

        if keypress.pressed(KeyCode::Space) {
            fire = true;
        }

        // we always set the value. Setting it to None means that the input was missing, it's not the same
        // as saying that the input was 'no keys pressed'
        action_state.0 = Inputs::Controls(Controls {
            joy_x: left - right,
            joy_y: up - down,
            fire,
        });
    }
}

/// The client input only gets applied to predicted entities that we own
/// This works because we only predict the user's controlled entity.
/// If we were predicting more entities, we would have to only apply movement to the player owned one.
fn player_movement(
    // timeline: Single<&LocalTimeline>,
    mut position_query: Query<(&mut PlayerPosition, &ActionState<Inputs>), With<Predicted>>,
) {
    // let tick = timeline.tick();
    for (position, input) in position_query.iter_mut() {
        // trace!(?tick, ?position, ?input, "client");
        // NOTE: be careful to directly pass Mut<PlayerPosition>
        // getting a mutable reference triggers change detection, unless you use `as_deref_mut()`
        shared::shared_movement_behaviour(position, input);
    }
}

/*
/// System to receive messages on the client
pub(crate) fn receive_message1(mut receiver: Single<&mut MessageReceiver<Message1>>) {
    for message in receiver.receive() {
        info!("Received message: {:?}", message);
    }
}
*/

/// When the predicted copy of the client-owned entity is spawned, do stuff
///
/// Note that this will be triggered multiple times: for the locally-controlled entity,
/// but also for the remote-controlled entities that are spawned with [`Interpolated`].
/// The `With<Predicted>` filter ensures we only add the `InputMarker` once.
pub(crate) fn handle_predicted_spawn(
    trigger: On<Add, PlayerId>,
    mut predicted: Query<&mut PlayerColor, With<Predicted>>,
    mut commands: Commands,
) {
    let entity = trigger.entity;
    if let Ok(mut color) = predicted.get_mut(entity) {
        /*
        let hsva = Hsva {
            saturation: 0.4,
            ..Hsva::from(color.0)
        };
        */
        color.0 = PresetColor::Red;
        warn!("Add InputMarker to Predicted entity: {:?}", entity);
        commands
            .entity(entity)
            .insert(InputMarker::<Inputs>::default());
    }
}

/// When the predicted copy of the client-owned entity is spawned, do stuff
pub(crate) fn handle_interpolated_spawn(
    trigger: On<Add, PlayerColor>,
    mut interpolated: Query<&mut PlayerColor, With<Interpolated>>,
) {
    if let Ok(mut color) = interpolated.get_mut(trigger.entity) {
        /*
        let hsva = Hsva {
            saturation: 0.1,
            ..Hsva::from(color.0)
        };
        */
        color.0 = PresetColor::Grey;
    }
}

pub fn spawn_connections(app: &mut App) {
    let client_id = gen_random_id();

    let _client = app
        .world_mut()
        .spawn(ClientSettingsThing { client_id })
        .id();

    app.add_systems(Startup, connect);
}

pub fn connect(mut commands: Commands, client: Single<Entity, With<Client>>) {
    commands.trigger(Connect {
        entity: client.into_inner(),
    });
}
