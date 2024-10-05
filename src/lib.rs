pub extern crate bevy_mod_transform2d;
pub extern crate bevy_rapier2d;
pub extern crate bevy_turborand;
use std::marker::PhantomData;

use bevy::prelude::{App, Plugin, ResMut, Startup, Vec2};
use bevy_mod_transform2d::{transform2d::Transform2d, Transform2dPlugin};

use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use bevy_turborand::prelude::RngPlugin;
use camera::CameraPlugin;
use meta_states::PluginControlState;
use stats::{Health, Knockback, Speed};

use self::{
    actors::ActorPlugin, ai::AIPlugin, meta_states::DummyStates, player::PlayerPlugin,
    projectile::ProjectilePlugin, weapons::WeaponPlugin,
};

pub mod actors;
pub mod ai;
pub mod camera;
pub mod meta_states;
pub mod player;
pub mod projectile;
pub mod stats;
pub mod transform2d_mods;
pub mod utils;
pub mod weapons;

pub fn TwinStickPlugin() -> TwinStickToggleablePlugin<DummyStates> {
    TwinStickToggleablePlugin::<DummyStates>::default()
}

pub struct TwinStickToggleablePlugin<T: PluginControlState> {
    pub use_default_camera: bool,
    pub _p: PhantomData<T>,
}

impl<T: PluginControlState> Default for TwinStickToggleablePlugin<T> {
    fn default() -> Self {
        TwinStickToggleablePlugin {
            use_default_camera: true,
            _p: PhantomData,
        }
    }
}

impl<T: PluginControlState> Plugin for TwinStickToggleablePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_state::<T>();
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.),
            RngPlugin::default(),
        ));

        app.add_plugins((
            Transform2dPlugin,
            PlayerPlugin::<T>::default(),
            ActorPlugin::<T>::default(),
            WeaponPlugin::<T>::default(),
            AIPlugin,
            ProjectilePlugin::<T>::default(),
        ));

        if self.use_default_camera {
            app.add_plugins(CameraPlugin);
        }

        app.register_type::<Transform2d>();
        app.register_type::<Speed>();
        app.register_type::<Health>();
        app.register_type::<Knockback>();

        app.add_systems(Startup, rapier_config_setup);
    }
}

fn rapier_config_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0., 0.);
}
