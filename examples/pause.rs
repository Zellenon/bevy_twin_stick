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

/// When the game is in this state, it will automatically enable all twin_stick functions.
/// Otherwise, everything time-based in the library will pause.
impl PluginControlState for PauseState {
    fn active_state() -> Self {
        PauseState::Moving
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_plugins(TwinStickToggleablePlugin::<PauseState>::default().use_default_camera(false));
    app.add_systems(OnEnter(PauseState::Locked), pause_physics);
    app.add_systems(OnEnter(PauseState::Moving), unpause_physics);

    app.add_systems(Startup, setup);
    app.add_systems(Update, toggle_pause);

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

pub(crate) fn toggle_pause(
    keyboard_input: Res<Input<KeyCode>>,
    mut pause_state: ResMut<NextState<PauseState>>,
    current_pause_state: Res<State<PauseState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Toggling Gamestate");
        match current_pause_state.get() {
            PauseState::Locked => pause_state.set(PauseState::Moving),
            PauseState::Moving => pause_state.set(PauseState::Locked),
        }
        dbg!("Current gamestate", current_pause_state.get());
    }
}

fn pause_physics(mut physics: ResMut<RapierConfiguration>) {
    (*physics).physics_pipeline_active = false;
}

fn unpause_physics(mut physics: ResMut<RapierConfiguration>) {
    (*physics).physics_pipeline_active = true;
}
