use bevy::{
    prelude::{info, Component, GlobalTransform, Query, Res, Transform, Vec3, With, Without},
    time::Time,
};
use big_brain::{
    prelude::ActionState,
    thinker::{ActionSpan, Actor},
};

use crate::{utils::closest, Hunger, resource::FoodSource};

// ACTIONS
const INTERACTION_DISTANCE: f32 = 0.1;

#[derive(Component, Debug, Clone)]
pub(crate) struct Eat {
    /// How much this action replenishes hunger over time.
    pub per_second: f32,
}

pub(crate) fn eat_action(
    time: Res<Time>,
    mut hungers: Query<(&mut Hunger, &GlobalTransform), Without<FoodSource>>,
    food_sources: Query<&GlobalTransform, With<FoodSource>>,
    mut eat_actions: Query<(&Actor, &mut ActionState, &Eat, &ActionSpan)>,
) {
    for (Actor(actor), mut state, eat, span) in &mut eat_actions {
        let _guard = span.span().enter();

        let (mut hunger, actor_pos) = hungers.get_mut(*actor).expect("This actor has no hunger");

        match *state {
            ActionState::Requested => {
                // Check if we are close enough before staring to eat.
                let mut points = food_sources.iter().map(|element| element.translation());
                let closest_food = closest(&mut points, actor_pos.translation());

                if (closest_food - actor_pos.translation()).length() <= INTERACTION_DISTANCE {
                    *state = ActionState::Executing;
                } else {
                    *state = ActionState::Failure;
                }
            }
            ActionState::Executing => {
                info!("Eating");
                // TODO: This should also remove food from the FoodSource
                // TODO: If food-source becomes empty, remove it from the game.
                hunger.value -= eat.per_second * time.delta_seconds();
                if hunger.value <= 0.0 {
                    hunger.value = 0.0;
                    *state = ActionState::Success;
                }
                // TODO: If the food-source we were eating from dissapeared, cancel the action
            }
            ActionState::Cancelled => *state = ActionState::Failure,
            ActionState::Success => {} // Play animation on the hunger-bar,
            _ => {}
        }
    }
}

#[derive(Component, Debug, Clone)]
pub(crate) struct MoveToFood {
    pub speed: f32,
}

pub(crate) fn move_to_food(
    time: Res<Time>,
    mut actor_positions: Query<(&mut Transform, &GlobalTransform), Without<FoodSource>>,
    food_sources: Query<&GlobalTransform, With<FoodSource>>,
    mut move_actions: Query<(&Actor, &mut ActionState, &MoveToFood, &ActionSpan)>,
) {
    for (Actor(actor), mut state, move_action, span) in &mut move_actions {
        let _guard = span.span().enter();

        match *state {
            ActionState::Requested => *state = ActionState::Executing, // Start looking for water
            ActionState::Executing => {
                info!("Moving to food source");
                let (mut transform, global_transform) = actor_positions.get_mut(*actor).unwrap();
                let closest = closest(
                    &mut food_sources.iter().map(|e| e.translation()),
                    global_transform.translation(),
                );
                let delta = closest - global_transform.translation();
                let distance = delta.length();

                if distance <= INTERACTION_DISTANCE {
                    *state = ActionState::Success;
                } else {
                    let step_size = time.delta_seconds() * move_action.speed;
                    let movement = delta.normalize() * step_size.min(distance);
                    transform.translation += movement;
                }
            }
            ActionState::Cancelled => *state = ActionState::Failure,
            _ => {}
        }
    }
}
