// SCORES

use bevy::prelude::{info, Changed, Component, Query, With};
use big_brain::{scorers::Score, thinker::Actor};

use super::needs::Hunger;

#[derive(Component, Debug, Clone)]
pub(crate) struct Hungry;

pub(crate) fn hungry_scorer(
    mut q: Query<(&Actor, &mut Score), With<Hungry>>,
    hungers: Query<&Hunger, Changed<Hunger>>,
) {
    for (Actor(actor), mut score) in &mut q {
        if let Ok(hunger) = hungers.get(*actor) {
            score.set(hunger.value / 100.0);
            info!("Hunger: {:?}", hunger.value);
        }
    }
}
