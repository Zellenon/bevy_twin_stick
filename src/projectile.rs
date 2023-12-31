use std::{marker::PhantomData, time::Duration};

use bevy::{
    prelude::{
        in_state, App, Bundle, Commands, Component, ComputedVisibility, DespawnRecursiveExt,
        Entity, Event, EventReader, EventWriter, GlobalTransform, IntoSystemConfigs, Plugin, Query,
        Res, Transform, Update, Vec2, Visibility,
    },
    time::{Time, Timer, TimerMode},
};
use bevy_mod_transform2d::transform2d::Transform2d;
use bevy_rapier2d::{
    pipeline::CollisionEvent,
    prelude::{
        ActiveEvents, Collider, ColliderMassProperties, ExternalImpulse, RigidBody, Velocity,
    },
};

use crate::meta_states::PluginControlState;

#[derive(Component)]
pub struct Lifespan(Timer);

impl Lifespan {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

impl Default for Lifespan {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(400), TimerMode::Once))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectileImpactBehavior {
    Die,
    Bounce,
}

#[derive(Component)]
pub struct Projectile {
    pub on_hit: ProjectileImpactBehavior,
    pub on_impact: ProjectileImpactBehavior,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            on_hit: ProjectileImpactBehavior::Die,
            on_impact: ProjectileImpactBehavior::Die,
        }
    }
}

#[derive(Component)]
pub struct Knockback(pub f32);

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub _transform: Transform,
    pub transform: Transform2d,
    pub global_transform: GlobalTransform,
    pub rigidbody: RigidBody,
    pub velocity: Velocity,
    pub mass_properties: ColliderMassProperties,
    pub collider: Collider,
    pub event_trigger: ActiveEvents,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            projectile: Projectile::default(),
            visibility: Visibility::Visible,
            computed_visibility: ComputedVisibility::default(),
            velocity: Default::default(),
            transform: Default::default(),
            _transform: Default::default(),
            global_transform: Default::default(),
            rigidbody: RigidBody::Dynamic,
            mass_properties: ColliderMassProperties::Density(1.),
            collider: Collider::ball(5.),
            event_trigger: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

#[derive(Event)]
pub struct KnockbackEvent {
    entity: Entity,
    direction: Vec2,
    force: f32,
}

#[derive(Event)]
pub struct ProjectileImpactEvent {
    pub projectile: Entity,
    pub impacted: Entity,
}

#[derive(Event)]
pub struct ProjectileClashEvent(pub Entity, pub Entity);

#[derive(Default)]
pub struct ProjectilePlugin<T: PluginControlState> {
    _z: PhantomData<T>,
}

impl<T: PluginControlState> Plugin for ProjectilePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<KnockbackEvent>();
        // .add_system(projectile_impact)
        app.add_systems(
            Update,
            (
                tick_lifetimes,
                knockback_events,
                projectile_event_dispatcher,
                kill_projectiles_post_impact,
                knockback_from_projectiles,
            )
                .run_if(in_state(T::active_state())),
        );

        app.add_event::<ProjectileImpactEvent>()
            .add_event::<ProjectileClashEvent>();
    }
}

fn tick_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    mut lifespans: Query<(&mut Lifespan, Entity)>,
) {
    for (mut lifespan, entity) in lifespans.iter_mut() {
        lifespan.0.tick(time.delta());

        if lifespan.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn projectile_event_dispatcher(
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<&Projectile>,
    mut projectile_events: EventWriter<ProjectileImpactEvent>,
    mut clash_events: EventWriter<ProjectileClashEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            match (projectile_query.get(*e1), projectile_query.get(*e2)) {
                (Ok(_), Ok(_)) => clash_events.send(ProjectileClashEvent(*e1, *e2)),
                (Ok(_), _) => projectile_events.send(ProjectileImpactEvent {
                    projectile: *e1,
                    impacted: *e2,
                }),
                (Err(_), Ok(_)) => projectile_events.send(ProjectileImpactEvent {
                    impacted: *e1,
                    projectile: *e2,
                }),
                (Err(_), Err(_)) => continue,
            };
        }
    }
}

fn knockback_from_projectiles(
    mut knockback_events: EventWriter<KnockbackEvent>,
    mut projectile_events: EventReader<ProjectileImpactEvent>,
    projectiles: Query<(&Knockback, Option<&Velocity>)>,
    positions: Query<&Transform2d>,
) {
    for ProjectileImpactEvent {
        projectile,
        impacted,
    } in projectile_events.iter()
    {
        if let Ok((Knockback(knockback), vel)) = projectiles.get(*projectile) {
            let hit_angle = positions.get(*projectile).unwrap().translation
                - positions.get(*impacted).unwrap().translation;
            knockback_events.send(KnockbackEvent {
                entity: *impacted,
                direction: match vel {
                    Some(Velocity { linvel, angvel: _ }) => hit_angle + *linvel,
                    None => hit_angle,
                },
                force: *knockback,
            })
        }
    }
}

fn kill_projectiles_post_impact(
    mut events: EventReader<ProjectileImpactEvent>,
    mut commands: Commands,
    query: Query<&Projectile>,
) {
    for ProjectileImpactEvent {
        projectile: projectile_id,
        impacted: _,
    } in events.iter()
    {
        let proj = query.get(*projectile_id);
        match proj {
            Ok(projectile) => {
                if projectile.on_impact == ProjectileImpactBehavior::Die {
                    commands.entity(*projectile_id).despawn_recursive();
                }
            }
            Err(_) => (),
        }
    }
}

fn knockback_events(
    mut knockback_events: EventReader<KnockbackEvent>,
    mut target_query: Query<&mut ExternalImpulse>,
) {
    for KnockbackEvent {
        entity,
        direction,
        force,
    } in knockback_events.iter()
    {
        let impulse_vector = Vec2::normalize(*direction) * *force;
        if let Ok(mut impulse) = target_query.get_mut(*entity) {
            impulse.impulse += impulse_vector;
        }
    }
}
