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

use bevy_persistent::Persistent;

#[derive(Copy, Clone, Debug)]
pub struct SharedSettings {
    /// An id to identify the protocol version
    pub protocol_id: u64,

    /// a 32-byte array to authenticate via the Netcode.io protocol
    pub private_key: [u8; 32],
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
pub fn shared_movement_behaviour(mut position: Mut<PlayerPosition>, input: &Inputs) {
    const MOVE_SPEED: f32 = 10.0;
    let Inputs::Controls(controls) = input else {
        return;
    };

    let x_squared = controls.joy_x * controls.joy_x;
    let y_squared = controls.joy_y * controls.joy_y;
    let x_circular = controls.joy_x * f32::sqrt(1.0 - 0.5 * y_squared);
    let y_circular = controls.joy_y * f32::sqrt(1.0 - 0.5 * x_squared);

    position.x += x_circular * MOVE_SPEED;
    position.y += y_circular * MOVE_SPEED;
}

#[derive(Resource, Serialize, Deserialize)]
struct SaveData;

// this is meant to be a component you can add to any Persistent<R> to autosave it, but it needs work
#[derive(Component)]
struct PersistentAutosave;

pub struct AutosavePlugin;

#[derive(Resource)]
struct SaveTimer(Timer);

impl Plugin for AutosavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SaveTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Update, persist_save_data);
    }
}

fn _update_save_data(_query: Query<&mut Persistent<SaveData>, With<PersistentAutosave>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say autosave
}

fn persist_save_data(
    time: Res<Time>,
    mut timer: ResMut<SaveTimer>,
    mut query: Query<&mut Persistent<SaveData>, With<PersistentAutosave>>,
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, autosave
    if timer.0.tick(time.delta()).just_finished() {
        for persistent in &mut query {
            persistent.persist().expect("failed to save data");
        }
    }
}
