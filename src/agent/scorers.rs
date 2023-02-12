// SCORES

use bevy::prelude::{Changed, Component, Query, With};
use big_brain::{prelude::ScorerBuilder, scorers::Score, thinker::Actor};

use crate::fauna::needs::{Hunger, Reproduction, Thirst};

#[derive(Component, Debug, Clone, ScorerBuilder)]
pub(crate) struct Hungry;

#[derive(Component, Debug, Clone, ScorerBuilder)]
pub(crate) struct Thirsty;

#[derive(Component, Debug, Clone, ScorerBuilder)]
pub(crate) struct ReproductionScore;

pub(crate) fn hungry_scorer(
    mut scorers: Query<(&Actor, &mut Score), With<Hungry>>,
    q: Query<&Hunger, Changed<Hunger>>,
) {
    for (Actor(actor), mut score) in &mut scorers {
        if let Ok(hunger) = q.get(*actor) {
            score.set(hunger.value / 100.0);
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
        }
    }
}

pub(crate) fn reproduction_scorer(
    mut scorers: Query<(&Actor, &mut Score), With<ReproductionScore>>,
    q: Query<&Reproduction, Changed<Reproduction>>,
) {
    for (Actor(actor), mut score) in &mut scorers {
        if let Ok(reproduction) = q.get(*actor) {
            score.set(if reproduction.value >= 100.0 {
                1.0
            } else {
                0.0
            });
        }
    }
}
