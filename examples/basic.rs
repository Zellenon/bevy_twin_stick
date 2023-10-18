use bevy::{
    app::App,
    ecs::system::EntityCommands,
    prelude::{default, ClearColor, Commands, Name, PluginGroup, Startup},
    render::color::Color,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_twin_stick::{
    actors::ActorBundle, ai::keyboard::KeyboardAI, bevy_rapier2d::render::RapierDebugRenderPlugin,
    player::Player, stats::Speed, TwinStickPlugin,
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(TwinStickPlugin);

    app.add_systems(Startup, setup);

    if cfg!(debug_assertions) {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }

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
        KeyboardAI,
    ));
}
