//shared.rs

use bevy::prelude::*;

use core::net::{IpAddr, Ipv4Addr, SocketAddr};
use core::time::Duration;

use crate::msg::Inputs;
use crate::msg::PlayerPosition;

pub const GAME_DIR_NAME: &'static str = "dmak26";

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;
pub const SERVER_PORT: u16 = 25252;
/// 0 means that the OS will assign any available port
pub const CLIENT_PORT: u16 = 0;
pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), SERVER_PORT);
pub const SHARED_SETTINGS: SharedSettings = SharedSettings {
    protocol_id: 0,
    private_key: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ],
};

pub const SEND_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Copy, Clone, Debug)]
pub struct SharedSettings {
    /// An id to identify the protocol version
    pub protocol_id: u64,

    /// a 32-byte array to authenticate via the Netcode.io protocol
    pub private_key: [u8; 32],
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        println!("peers:");
        for name in &query {
            println!("{},", name.0);
        }
    }
}

pub struct CounterPlugin;

impl Plugin for CounterPlugin {
    fn build(&self, app: &mut App) {
        use dirs;

        use std::path::Path;

        use bevy_persistent::prelude::*;

        // local save data
        let data_dir = dirs::data_dir()
            .map(|native_dir| native_dir.join(GAME_DIR_NAME))
            .unwrap_or(Path::new("local").join("data"));

        /*
        let config_dir = dirs::config_dir()
            .map(|native_dir| native_dir.join(shared::GAME_DIR_NAME))
            .unwrap_or(Path::new("local").join("config"));
        */

        println!("data={:?}", data_dir);
        //println!("config={:?}", config_dir);

        app.insert_resource(
            Persistent::<Counter>::builder()
                .name("Dreams")
                .format(StorageFormat::RonPretty)
                .path(data_dir.join("dreams.sav"))
                .default(Counter::default())
                .build()
                .expect("failed to init save data"),
        );
    }
}

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Resource, Serialize, Deserialize)]
pub struct Counter(i64);

impl Counter {
    pub fn inc(&mut self) -> i64 {
        self.0 += 1;
        self.0
    }

    pub fn get_count(&self) -> i64 {
        self.0
    }
}

// This system defines how we update the player's positions when we receive an input
pub(crate) fn shared_movement_behaviour(mut position: Mut<PlayerPosition>, input: &Inputs) {
    const MOVE_SPEED: f32 = 10.0;
    let Inputs::Direction(direction) = input else {
        return;
    };
    if direction.up {
        position.y += MOVE_SPEED;
    }
    if direction.down {
        position.y -= MOVE_SPEED;
    }
    if direction.left {
        position.x -= MOVE_SPEED;
    }
    if direction.right {
        position.x += MOVE_SPEED;
    }
}
