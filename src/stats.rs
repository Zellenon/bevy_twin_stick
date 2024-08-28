use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct Speed(pub f32);

#[derive(Component, Reflect, Clone, Copy, PartialEq, Debug)]
pub struct Health(pub f32);

#[derive(Component, Reflect, Clone, Copy, PartialEq, Debug)]
pub struct Knockback(pub f32);
