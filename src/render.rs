use bevy::prelude::*;

use bevy::input::common_conditions::input_just_pressed;

use crate::shared::Counter;

#[derive(Component)]
struct MenuMusic;

//#[derive(Resource)]
//struct CoinSound(AudioSource);

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
                    //font: font_handle.clone().into(),
                    font_size: 60.0,
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("DREAMS"),
                    TextLayout::new_with_justify(Justify::Center),
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
        mut number: ResMut<Counter>,
        //~ coin_sound: Option<Res<CoinSound>>,
        asset_server: Res<AssetServer>,
    ) {
        number.inc();

        commands.spawn((
            AudioPlayer::new(asset_server.load("coin1.ogg")),
            PlaybackSettings::ONCE,
        ));
    }

    fn update_counter_text(counter: Res<Counter>, mut text: Single<&mut Text, With<CounterText>>) {
        text.0 = format!("{}", counter.get());
    }
}
