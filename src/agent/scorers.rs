// SCORES

use bevy::prelude::{info, Changed, Component, Query, With};
use big_brain::{scorers::Score, thinker::Actor};

use super::needs::{Hunger, Thirst};

#[derive(Component, Debug, Clone)]
pub(crate) struct Hungry;

#[derive(Component, Debug, Clone)]
pub(crate) struct Thirsty;

pub(crate) fn hungry_scorer(
    mut scorers: Query<(&Actor, &mut Score), With<Hungry>>,
    q: Query<&Hunger, Changed<Hunger>>,
) {
    for (Actor(actor), mut score) in &mut scorers {
        if let Ok(hunger) = q.get(*actor) {
            score.set(hunger.value / 100.0);
            info!("Hunger: {:?}", hunger.value);
        }
    }
}

pub(crate) fn thirsty_scorer(
    mut scorers: Query<(&Actor, &mut Score), With<Thirsty>>,
    q: Query<&Thirst, Changed<Thirst>>,
) {
    for (Actor(actor), mut score) in &mut scorers {
        if let Ok(thirst) = q.get(*actor) {
            score.set(thirst.value / 100.0);
            info!("Thirst: {:?}", thirst.value);
        }
    }
}
