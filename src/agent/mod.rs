use bevy::prelude::{IntoSystemConfigs, Plugin};
use big_brain::{BigBrainPlugin, BigBrainSet};

use self::{
    actions::{
        drink_action, eat_action, find_drink, find_food, idle_action, move_to_target,
        reproduce_action,
    },
    scorers::{hungry_scorer, reproduction_scorer, thirsty_scorer},
};

pub(crate) mod actions;
pub(crate) mod scorers;

pub(crate) struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BigBrainPlugin)
            .add_systems(
                (
                    find_food,
                    find_drink,
                    eat_action,
                    drink_action,
                    move_to_target,
                    reproduce_action,
                    idle_action,
                )
                    .in_set(BigBrainSet::Actions),
            )
            .add_systems(
                (hungry_scorer, thirsty_scorer, reproduction_scorer).in_set(BigBrainSet::Scorers),
            );
    }
}
