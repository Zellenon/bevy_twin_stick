use bevy::{
    app::App,
    prelude::{
        ClearColor, Commands, Input, KeyCode, Name, NextState, OnEnter, Res, ResMut, Startup,
        State, States, Update,
    },
    reflect::Reflect,
    render::color::Color,
    DefaultPlugins,
};
use bevy_rapier2d::prelude::RapierConfiguration;
use bevy_twin_stick::{
    actors::{ActorBundle, Faction},
    ai::{keyboard::KeyboardAI, wander::PerlinWanderAI},
    bevy_rapier2d::render::RapierDebugRenderPlugin,
    meta_states::PluginControlState,
    player::Player,
    stats::Speed,
    TwinStickToggleablePlugin,
};

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
enum PauseState {
    Locked,
    #[default]
    Moving,
}

impl PluginControlState for PauseState {
    fn active_state() -> Self {
        PauseState::Moving
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_plugins(TwinStickToggleablePlugin::<PauseState>::default());
    app.add_systems(OnEnter(PauseState::Locked), pause);
    app.add_systems(OnEnter(PauseState::Moving), unpause);

    app.add_systems(Startup, setup);
    app.add_systems(Update, toggle_camera_state);

    app.insert_resource(ClearColor(Color::rgb(
        0xA9 as f32 / 255.0,
        0xA9 as f32 / 255.0,
        0xAF as f32 / 255.0,
    )));

    app.run();
    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Player,
        Name::new("Player"),
        ActorBundle::default(),
        Speed(1000.),
        KeyboardAI,
    ));

    commands.spawn((
        Name::new("wanderer"),
        ActorBundle {
            faction: Faction::FactionID(1),
            ..Default::default()
        },
        Speed(200.),
        PerlinWanderAI::new(0.4, 0.8, 0.01, 0.95),
    ));
}

pub(crate) fn toggle_camera_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_setting: ResMut<NextState<PauseState>>,
    current_camera_setting: Res<State<PauseState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Toggling Gamestate");
        match current_camera_setting.get() {
            PauseState::Locked => camera_setting.set(PauseState::Moving),
            PauseState::Moving => camera_setting.set(PauseState::Locked),
        }
        dbg!("Current gamestate", current_camera_setting.get());
    }
}

fn pause(mut physics: ResMut<RapierConfiguration>) {
    (*physics).physics_pipeline_active = false;
}

fn unpause(mut physics: ResMut<RapierConfiguration>) {
    (*physics).physics_pipeline_active = true;
}
