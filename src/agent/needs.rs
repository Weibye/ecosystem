use bevy::{
    prelude::{Component, Query, Res, Changed, Entity, Commands, warn},
    time::Time,
};

#[derive(Component, Debug, Copy, Clone)]
pub(crate) struct Hunger {
    /// How fast the entity gets hungry.
    pub per_second: f32,
    /// Current value of the hunger.
    pub value: f32,
}

#[derive(Component, Debug, Copy, Clone)]
pub(crate) struct Thirst {
    pub per_second: f32,
    pub value: f32,
}

#[derive(Component, Debug, Copy, Clone)]
pub(crate) struct Health {
    pub value: f32
}

/// System that decays all agents' hunger over time.
pub(crate) fn hunger_decay(time: Res<Time>, mut q: Query<&mut Hunger>) {
    for mut hunger in &mut q {
        hunger.value += hunger.per_second * time.delta_seconds();

        if hunger.value >= 100.0 {
            hunger.value = 100.0;
        }
    }
}

/// System that decays all agents' hunger over time.
pub(crate) fn thirst_decay(time: Res<Time>, mut q: Query<&mut Thirst>) {
    for mut thirst in &mut q {
        thirst.value += thirst.per_second * time.delta_seconds();

        if thirst.value >= 100.0 {
            thirst.value = 100.0;
        }
    }
}

pub(crate) fn health_update(mut q: Query<(&mut Health, &Hunger, &Thirst)>) {
    for (mut health, hunger, thirst) in &mut q {
        let hunger_mod = if hunger.value >= 80.0 { -0.5 } else { 0.5 };
        let thirst_mod = if thirst.value >= 80.0 { -1.0 } else { 1.0 };
        
        health.value += hunger_mod + thirst_mod;

        if health.value >= 100.0 { health.value = 100.0 }
    }
}

/// System that will despawn any entity that reaches zero health.
pub(crate) fn death(mut cmd: Commands, q: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, health) in &q {
        if health.value <= 0.0 {
            warn!("{:?} died.", entity);
            cmd.entity(entity).despawn();
        }
    }
}