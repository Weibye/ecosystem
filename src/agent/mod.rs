use bevy::prelude::Plugin;
use big_brain::{BigBrainPlugin, BigBrainStage};

use self::{
    actions::{eat_action, find_food, move_to_target},
    scorers::hungry_scorer,
};

pub(crate) mod actions;
pub(crate) mod needs;
pub(crate) mod scorers;

pub(crate) struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BigBrainPlugin)
            .add_system_to_stage(BigBrainStage::Actions, eat_action)
            .add_system_to_stage(BigBrainStage::Actions, find_food)
            .add_system_to_stage(BigBrainStage::Actions, move_to_target)
            .add_system_to_stage(BigBrainStage::Scorers, hungry_scorer);
    }
}
