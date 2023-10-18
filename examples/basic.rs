use bevy::{
    app::App,
    prelude::{ClearColor, Commands, Name, Startup, Vec2},
    render::color::Color,
    DefaultPlugins,
};
use bevy_mod_transform2d::transform2d::Transform2d;
use bevy_rapier2d::prelude::{Collider, RigidBody};
use bevy_twin_stick::{
    actors::ActorBundle, ai::keyboard::KeyboardAI, bevy_rapier2d::render::RapierDebugRenderPlugin,
    player::Player, stats::Speed, transform2d_mods::Sprite2dBundle, TwinStickPlugin,
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(TwinStickPlugin::default());
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_systems(Startup, setup);

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
        Sprite2dBundle {
            transform: Transform2d::from_translation(Vec2::new(15., 15.)),
            visibility: bevy::prelude::Visibility::Visible,
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(5., 5.),
    ));
}
