use bevy::prelude::Plugin;
use big_brain::{BigBrainPlugin, BigBrainStage};

use self::{
    actions::{drink_action, eat_action, find_drink, find_food, move_to_target},
    needs::{death, health_update, hunger_decay, thirst_decay},
    scorers::{hungry_scorer, thirsty_scorer},
};

pub(crate) mod actions;
pub(crate) mod needs;
pub(crate) mod scorers;

pub(crate) struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BigBrainPlugin)
            .add_system_to_stage(BigBrainStage::Actions, find_food)
            .add_system_to_stage(BigBrainStage::Actions, find_drink)
            .add_system_to_stage(BigBrainStage::Actions, eat_action)
            .add_system_to_stage(BigBrainStage::Actions, drink_action)
            .add_system_to_stage(BigBrainStage::Actions, move_to_target)
            .add_system_to_stage(BigBrainStage::Scorers, hungry_scorer)
            .add_system_to_stage(BigBrainStage::Scorers, thirsty_scorer)
            .add_system(hunger_decay)
            .add_system(thirst_decay)
            .add_system(health_update)
            .add_system(death);
    }
}
