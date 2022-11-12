use bevy::{
    prelude::{
        info, Commands, Component, Entity, GlobalTransform, Query, Res, Transform, Vec3, With,
    },
    time::Time,
};
use big_brain::{
    prelude::ActionState,
    thinker::{ActionSpan, Actor},
};

use crate::resource::FoodSource;

use super::needs::Hunger;

const INTERACTION_DISTANCE: f32 = 0.1;

// ACTIONS

/// Action that moves to a target.
#[derive(Component, Debug, Clone)]
pub(crate) struct MoveAction;

/// Action that figures out which food-source to get.
#[derive(Component, Debug, Clone)]
pub(crate) struct FindFoodAction;

/// Action that eats from a food source.
#[derive(Component, Debug, Clone)]
pub(crate) struct EatAction;

// Action targets

/// Component that contains the data of which food-source to eat.
#[derive(Component, Debug, Clone)]
pub(crate) struct EatTarget {
    /// Which entity to eat
    pub target: Entity,
}

/// Component that contians the data of where to move to.
#[derive(Component, Debug)]
pub(crate) struct MovementTarget {
    target: Vec3,
}

// Action abilities

// TODO: Observe ability

/// Marker component that an entity can move.
#[derive(Component, Debug)]
pub(crate) struct MoveAbility {
    pub speed: f32,
}

/// Marker component that an entity can eat food.
#[derive(Component, Debug)]
pub(crate) struct EatAbility {
    pub speed: f32,
}

/// Defines how to eat from food sources.
pub(crate) fn eat_action(
    mut cmd: Commands,
    time: Res<Time>,
    mut eaters: Query<(&mut Hunger, &EatAbility, &EatTarget)>,
    mut food_sources: Query<&mut FoodSource>,
    mut eat_actions: Query<(&Actor, &mut ActionState, &ActionSpan), With<EatAction>>,
) {
    for (Actor(actor), mut state, _) in &mut eat_actions {
        // let _guard = span.span().enter();

        match *state {
            ActionState::Requested => *state = ActionState::Executing,
            ActionState::Executing => {
                info!("Eating");

                if let Ok((mut hunger, eating_ability, eat_target)) = eaters.get_mut(*actor) {
                    if let Ok(mut food_source) = food_sources.get_mut(eat_target.target) {
                        // If there's no more food, cancel the eating action.
                        if food_source.content <= 0. {
                            info!("No more food available.");
                            *state = ActionState::Cancelled;
                        }

                        hunger.value -= eating_ability.speed * time.delta_seconds();
                        food_source.content -= eating_ability.speed * time.delta_seconds();

                        if hunger.value <= 0.0 {
                            hunger.value = 0.0;
                            *state = ActionState::Success;
                        }
                    } else {
                        info!("The food has disappeared.");
                        *state = ActionState::Cancelled;
                    }
                } else {
                    info!("No entities exist to perform this action");
                    *state = ActionState::Cancelled;
                }
            }
            ActionState::Cancelled => *state = ActionState::Failure,
            ActionState::Success => {
                info!("Eating completed");
                cmd.entity(*actor).remove::<EatTarget>();
            }
            _ => {}
        }
    }
}

/// Defines how an aget should move to a supplied target.
pub(crate) fn move_to_target(
    mut cmd: Commands,
    time: Res<Time>,
    mut q: Query<(&mut Transform, &MovementTarget, &MoveAbility)>,
    mut actions: Query<(&Actor, &mut ActionState, &ActionSpan), With<MoveAction>>,
) {
    for (Actor(actor), mut state, _) in &mut actions {
        match *state {
            ActionState::Requested => *state = ActionState::Executing,
            ActionState::Cancelled => *state = ActionState::Failure,
            ActionState::Executing => {
                info!("Moving to target");
                if let Ok((mut transform, target, ability)) = q.get_mut(*actor) {
                    let delta = target.target - transform.translation;
                    let distance = delta.length();

                    if distance <= INTERACTION_DISTANCE {
                        *state = ActionState::Success;
                    } else {
                        let step_size = time.delta_seconds() * ability.speed;
                        let movement = delta.normalize() * step_size.min(distance);
                        transform.translation += movement;
                    }
                } else {
                    info!("No entities exist to perform this action");
                    *state = ActionState::Cancelled;
                }
            }
            ActionState::Success => {
                info!("Target reached");
                cmd.entity(*actor).remove::<MovementTarget>();
            }
            _ => {}
        }
    }
}

/// Defines how an agent should look for a food source.
pub(crate) fn find_food(
    mut cmd: Commands,
    q: Query<&GlobalTransform, With<EatAbility>>,
    food_sources: Query<(Entity, &GlobalTransform), With<FoodSource>>,
    mut actions: Query<(&Actor, &mut ActionState, &ActionSpan), With<FindFoodAction>>,
) {
    for (Actor(actor), mut state, _) in &mut actions {
        match *state {
            ActionState::Requested => *state = ActionState::Executing,
            ActionState::Executing => {
                info!("Looking for food");
                if let Ok(transform) = q.get(*actor) {
                    // get the food source closes to the agent's current location
                    if let Some((food_source, food_source_pos)) =
                        food_sources.iter().min_by(|(_, ta), (_, tb)| {
                            let a_distance =
                                (ta.translation() - transform.translation()).length_squared();
                            let b_distance =
                                (tb.translation() - transform.translation()).length_squared();
                            a_distance.partial_cmp(&b_distance).unwrap()
                        })
                    {
                        cmd.entity(*actor)
                            .insert(MovementTarget {
                                target: food_source_pos.translation(),
                            })
                            .insert(EatTarget {
                                target: food_source,
                            });

                        *state = ActionState::Success;
                    } else {
                        info!("No food sources are closest");
                        *state = ActionState::Cancelled;
                    }
                } else {
                    info!("No entities exist to perform this action");
                    *state = ActionState::Cancelled;
                }
            }
            ActionState::Success => {
                info!("Found food source!");
            }
            ActionState::Cancelled => *state = ActionState::Failure,
            _ => {}
        }
    }
}
