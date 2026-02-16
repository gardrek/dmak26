// stuff for lightyear

// defining the protocol for messages and sync and such

use bevy::prelude::*;

use bevy::ecs::entity::MapEntities;

use lightyear::prelude::*;

use input::native::InputPlugin;

use serde::{Deserialize, Serialize};

// component protocol

/// A component that will identify each player internally
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(PeerId);

/// A component that will store the Dreams banked by all players
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Dreams(i64);

/// A component that each player can customize to identify themself
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerName(String);

/// A component that will store the color of the player's avatar
#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PlayerColor(pub PresetColor);

/// A component that will store the color of the player's avatar
#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq, Copy)]
pub enum PresetColor {
    Red = 1,
    Green,
    Yellow,
    Blue,
    Orange,
    Purple,
    Cyan,
    Magenta,
    Lime,
    Pink,
    Teal,
    Lavender,
    Brown,
    Beige,
    Maroon,
    Mint,
    Olive,
    Apricot,
    Navy,
    Grey,
}

#[cfg(feature = "client")]
impl From<PresetColor> for Srgba {
    fn from(other: PresetColor) -> Srgba {
        use PresetColor::*;

        match other {
            Red => Srgba::hex("e6194B"),
            Green => Srgba::hex("3cb44b"),
            Yellow => Srgba::hex("ffe119"),
            Blue => Srgba::hex("4363d8"),
            Orange => Srgba::hex("f58231"),
            Purple => Srgba::hex("911eb4"),
            Cyan => Srgba::hex("42d4f4"),
            Magenta => Srgba::hex("f032e6"),
            Lime => Srgba::hex("bfef45"),
            Pink => Srgba::hex("fabed4"),
            Teal => Srgba::hex("469990"),
            Lavender => Srgba::hex("dcbeff"),
            Brown => Srgba::hex("9A6324"),
            Beige => Srgba::hex("fffac8"),
            Maroon => Srgba::hex("800000"),
            Mint => Srgba::hex("aaffc3"),
            Olive => Srgba::hex("808000"),
            Apricot => Srgba::hex("ffd8b1"),
            Navy => Srgba::hex("000075"),
            Grey => Srgba::hex("a9a9a9"),
        }
        .unwrap()
    }
}

impl TryFrom<u64> for PresetColor {
    type Error = ();
    fn try_from(other: u64) -> Result<PresetColor, ()> {
        use PresetColor::*;
        Ok(match other {
            1 => Red,
            2 => Green,
            3 => Yellow,
            4 => Blue,
            5 => Orange,
            6 => Purple,
            7 => Cyan,
            8 => Magenta,
            9 => Lime,
            10 => Pink,
            11 => Teal,
            12 => Lavender,
            13 => Brown,
            14 => Beige,
            15 => Maroon,
            16 => Mint,
            17 => Olive,
            18 => Apricot,
            19 => Navy,
            20 => Grey,
            _ => return Err(()),
        })
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect, Deref, DerefMut)]
pub struct PlayerPosition(pub Vec2);

impl Ease for PlayerPosition {
    fn interpolating_curve_unbounded(start: Self, end: Self) -> impl Curve<Self> {
        FunctionCurve::new(Interval::UNIT, move |t| {
            PlayerPosition(Vec2::lerp(start.0, end.0, t))
        })
    }
}

// Player
#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    id: PlayerId,
    position: PlayerPosition,
    color: PlayerColor,
}

impl PlayerBundle {
    pub(crate) fn new(id: PeerId, position: Vec2) -> Self {
        // Generate pseudo random color from client id.
        let color = (((id.to_bits().wrapping_mul(30)) % 20) + 1)
            .try_into()
            .unwrap();
        Self {
            id: PlayerId(id),
            position: PlayerPosition(position),
            color: PlayerColor(color),
        }
    }
}

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.register_component::<PlayerId>();

        app.register_component::<Dreams>();

        app.register_component::<PresetColor>();

        app.add_plugins(InputPlugin::<Inputs>::default());

        app.register_component::<PlayerPosition>()
            .add_prediction()
            .add_linear_interpolation();
    }
}

// Inputs - the user actions sent to the server

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, Default)]
pub struct Controls {
    pub joy_x: f32,
    pub joy_y: f32,
    pub fire: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Reflect, Clone)]
pub enum Inputs {
    Controls(Controls),
    Spawn,
    Delete,
    ClickerPressed,
}

impl Default for Inputs {
    fn default() -> Self {
        Self::Controls(Controls::default())
    }
}

// All inputs need to implement the `MapEntities` trait
impl MapEntities for Inputs {
    fn map_entities<M: EntityMapper>(&mut self, _entity_mapper: &mut M) {}
}

// Channels
pub struct Channel1;

// Messages

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize);
