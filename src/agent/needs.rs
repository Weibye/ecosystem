use bevy::{
    prelude::{Component, Query, Res},
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
