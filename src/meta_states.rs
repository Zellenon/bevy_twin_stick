use bevy::prelude::Reflect;
use bevy::prelude::States;

pub trait PluginControlState: Default + States + Send + Sync + 'static {
    fn active_state() -> Self;
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug, States, Hash, Default)]
pub(crate) enum DummyStates {
    #[default]
    AlwaysActive,
}

impl PluginControlState for DummyStates {
    fn active_state() -> Self {
        Self::AlwaysActive
    }
}
