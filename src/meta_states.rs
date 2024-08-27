use bevy::prelude::States;

pub trait PluginControlState: Default + States + Send + Sync + 'static {
    fn active_state() -> Self;
}

#[derive(States, Debug, Hash, Default, Eq, PartialEq, Clone)]
pub(crate) enum DummyStates {
    #[default]
    AlwaysActive,
}

impl PluginControlState for DummyStates {
    fn active_state() -> Self {
        Self::AlwaysActive
    }
}
