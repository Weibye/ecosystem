use bevy::prelude::Plugin;
use big_brain::{BigBrainPlugin, BigBrainStage};

use self::{
    actions::{eat_action, move_to_food},
    scorers::hungry_scorer,
};

pub(crate) mod actions;
pub(crate) mod scorers;

pub(crate) struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BigBrainPlugin)
            .add_system_to_stage(BigBrainStage::Actions, eat_action)
            .add_system_to_stage(BigBrainStage::Actions, move_to_food)
            .add_system_to_stage(BigBrainStage::Scorers, hungry_scorer);
    }
}
