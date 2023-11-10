use std::marker::PhantomData;

use crate::meta_states::PluginControlState;
use crate::utils::*;
use crate::weapons::{FireWeaponEvent, Weapon, WeaponFireMode};

use bevy::prelude::{
    default, in_state, App, Camera2dBundle, Children, Commands, Component, Entity, EventWriter,
    Input, IntoSystemConfigs, MouseButton, Name, OrthographicProjection, Plugin, Query, Res,
    Resource, Startup, Update, With,
};
use bevy::window::Window;
use bevy_mod_transform2d::prelude::Spatial2dBundle;
use bevy_mod_transform2d::transform2d::Transform2d;

#[derive(Component, Resource)]
pub struct MainCamera(pub Entity);

#[derive(Component)]
pub struct TwinStickCamera;

#[derive(Component)]
pub struct CursorTracker;

#[derive(Component, Resource)]
pub struct Cursor(pub Entity);

#[derive(Component)]
pub struct Player;

#[derive(Default)]
pub struct PlayerPlugin<T: PluginControlState> {
    _z: PhantomData<T>,
}

impl<T: PluginControlState> Plugin for PlayerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(
            Update,
            (
                update_cursor_tracker,
                fire_player_weapons.run_if(player_exists),
            )
                .run_if(in_state(T::active_state())),
        );
    }
}

pub fn player_setup(mut commands: Commands) {
    let camera_entity = commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                ..default()
            },
            ..Default::default()
        })
        .insert(Name::new("Twin-Stick Player Camera"))
        .insert(Transform2d {
            z_translation: 100.,
            ..Default::default()
        })
        .insert(TwinStickCamera)
        .id();
    commands.insert_resource(MainCamera(camera_entity));

    let cursor_entity = commands
        .spawn(Spatial2dBundle::default())
        .insert(Name::new("Cursor"))
        .insert(CursorTracker)
        .id();
    commands.insert_resource(Cursor(cursor_entity));
}

pub fn update_cursor_tracker(
    mut transforms: Query<&mut Transform2d>,
    windows: Query<&Window>,
    cam: Res<MainCamera>,
    cursor: Res<Cursor>,
) {
    let camera_transform = transforms.get(cam.0).unwrap().clone();
    let mut cursor_transform = transforms.get_mut(cursor.0).unwrap();
    let window = windows.single();

    if let Some(_position) = window.cursor_position() {
        let new_cursor_pos = screen_to_world(_position, &camera_transform, window);
        cursor_transform.translation = new_cursor_pos;
    }
}

pub fn player_exists(players: Query<(), With<Player>>) -> bool {
    players.iter().count() > 0
}

pub fn fire_player_weapons(
    buttons: Res<Input<MouseButton>>,
    mut events: EventWriter<FireWeaponEvent>,
    weapons: Query<(Entity, &Weapon)>,
    players_children_query: Query<&Children, With<Player>>,
) {
    for parent_player in players_children_query.iter() {
        for &child in parent_player.iter() {
            if let Ok((entity, weapon)) = weapons.get(child) {
                let trigger_func = weapon.fire_mode;
                if ((buttons.just_pressed(MouseButton::Left)
                    && trigger_func == WeaponFireMode::SemiAuto)
                    || (buttons.pressed(MouseButton::Left)
                        && trigger_func == WeaponFireMode::FullAuto))
                    && weapon.can_fire
                {
                    events.send(FireWeaponEvent {
                        weapon: entity,
                        target: None,
                    })
                }
            }
        }
    }
}
