//

use bevy::prelude::*;

//use lightyear::prelude::*;

use dmak26::render;
use dmak26::shared;

//use dmak26::msg;
//use dmak26::shared::*;

// #[cfg(not(target_family = "wasm"))]
use bevy::asset::uuid::Uuid;

fn main() {
    println!("this is a client!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(shared::HelloPlugin)
        .add_plugins(shared::CounterPlugin)
        .add_plugins(render::CounterRenderPlugin)
        .add_plugins(ClientPlugin)
        .run();
}

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup);
    }
}

impl ClientPlugin {
    fn setup(mut _commands: Commands, _asset_server: Res<AssetServer>) {
        let _id = gen_random_id();
    }
}

//~ #[cfg(not(target_family = "wasm"))]
fn gen_random_id() -> u64 {
    let uuid = Uuid::new_v4();

    let (a, b) = uuid.as_u64_pair();

    a ^ b
}
