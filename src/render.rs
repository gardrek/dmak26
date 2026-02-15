// this module should handle video and audio for the client

use bevy::prelude::*;

use bevy::input::common_conditions::input_just_pressed;

use bevy_persistent::Persistent;

use crate::shared::Counter;

#[derive(Component)]
struct MenuMusic;

#[derive(Resource)]
struct CoinSound(Handle<AudioSource>);

/*
#[derive(Resource)]
struct LegendaryDeedsFont(());
*/

#[derive(Component)]
struct CounterText;

pub struct CounterRenderPlugin;

impl Plugin for CounterRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup);
        app.add_systems(
            Update,
            (
                Self::number_go_up.run_if(input_just_pressed(KeyCode::Space)),
                Self::update_counter_text,
            )
                .chain(),
        );
    }
}

impl CounterRenderPlugin {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            AudioPlayer::new(asset_server.load("game_menu_looping.ogg")),
            MenuMusic,
            PlaybackSettings::LOOP,
        ));

        let coin_sound = asset_server.load("coin1.ogg");

        commands.insert_resource(CoinSound(coin_sound));

        let font_handle = asset_server.load("ld.ttf");

        // With spans
        commands
            .spawn((
                Text::new("..."),
                TextLayout::new_with_justify(Justify::Center),
                CounterText,
                Node {
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    align_content: AlignContent::End,
                    ..default()
                },
                TextFont {
                    font: font_handle.clone().into(),
                    font_size: 60.0,
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("DREAMS"),
                    TextLayout::new_with_justify(Justify::Center),
                    TextFont {
                        font: font_handle.clone().into(),
                        ..Default::default()
                    },
                    Node {
                        top: px(75.0),
                        ..default()
                    },
                ));
            });

        /*
        commands.spawn((
            Text::new(""),
            Node {
                position_type: PositionType::Absolute,
                top: px(12),
                left: px(12),
                ..default()
            },
            CounterText,
        ));*/

        commands.spawn(Camera2d::default());
    }

    fn number_go_up(
        mut commands: Commands,
        mut number: ResMut<Persistent<Counter>>,
        coin_sound: Res<CoinSound>,
        //~ asset_server: Res<AssetServer>,
    ) {
        number
            .update(|counter| {
                counter.inc();
            })
            .unwrap();

        commands.spawn((
            AudioPlayer::new(coin_sound.0.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }

    fn update_counter_text(
        counter: Res<Persistent<Counter>>,
        mut text: Single<&mut Text, With<CounterText>>,
    ) {
        text.0 = format!("{}", counter.get_count());
    }
}
