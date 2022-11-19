use bevy::{
    prelude::{
        info, Commands, Component, Entity, EventWriter, GlobalTransform, Query, Res, Transform,
        Vec3, With,
    },
    time::Time,
};
use big_brain::{
    prelude::ActionState,
    thinker::{ActionSpan, Actor},
};

use crate::{
    fauna::{
        needs::{Hunger, Reproduction, Thirst},
        SpawnFauna,
    },
    map::{world_to_pos, TileSettings},
    resource::{FoodSource, WaterSource},
};

const INTERACTION_DISTANCE: f32 = 0.1;

// ACTIONS

/// Action that moves to a target.
#[derive(Component, Debug, Clone)]
pub(crate) struct MoveAction;

/// Action that figures out which food-source to get.
#[derive(Component, Debug, Clone)]
pub(crate) struct FindFoodAction;

/// Action that figures out which water-source to get.
#[derive(Component, Debug, Clone)]
pub(crate) struct FindDrinkAction;

/// Action that eats from a food source.
#[derive(Component, Debug, Clone)]
pub(crate) struct EatAction;

/// Action that drinks from a water source.
#[derive(Component, Debug, Clone)]
pub(crate) struct DrinkAction;

/// Action that reproduces and spawn an offspring.
#[derive(Component, Debug, Clone)]
pub(crate) struct ReproduceAction;

// Action targets

/// Component that contains the data of which food-source to eat.
#[derive(Component, Debug, Clone)]
pub(crate) struct EatTarget {
    /// Which entity to eat
    pub target: Entity,
}

/// Component that contains the data of which water-source to drink.
#[derive(Component, Debug, Clone)]
pub(crate) struct DrinkTarget {
    /// Which entity to drink
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

/// Marker component that an entity can eat food.
#[derive(Component, Debug)]
pub(crate) struct DrinkAbility {
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

/// Defines how to eat from food sources.
pub(crate) fn drink_action(
    mut cmd: Commands,
    time: Res<Time>,
    mut drinkers: Query<(&mut Thirst, &DrinkAbility, &DrinkTarget)>,
    mut water_sources: Query<&mut WaterSource>,
    mut drink_actions: Query<(&Actor, &mut ActionState, &ActionSpan), With<DrinkAction>>,
) {
    for (Actor(actor), mut state, _) in &mut drink_actions {
        // let _guard = span.span().enter();

        match *state {
            ActionState::Requested => *state = ActionState::Executing,
            ActionState::Executing => {
                info!("Drinking");

                if let Ok((mut thirst, drinking_ability, drink_target)) = drinkers.get_mut(*actor) {
                    if let Ok(mut water_source) = water_sources.get_mut(drink_target.target) {
                        // If there's no more food, cancel the eating action.
                        if water_source.content <= 0. {
                            info!("No more water available.");
                            *state = ActionState::Cancelled;
                        }

                        thirst.value -= drinking_ability.speed * time.delta_seconds();
                        water_source.content -= drinking_ability.speed * time.delta_seconds();

                        if thirst.value <= 0.0 {
                            thirst.value = 0.0;
                            *state = ActionState::Success;
                        }
                    } else {
                        info!("The water has disappeared.");
                        *state = ActionState::Cancelled;
                    }
                } else {
                    info!("No entities exist to perform this action");
                    *state = ActionState::Cancelled;
                }
            }
            ActionState::Cancelled => *state = ActionState::Failure,
            ActionState::Success => {
                info!("Drinking completed");
                cmd.entity(*actor).remove::<DrinkTarget>();
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
                        let pos = food_source_pos.translation();
                        cmd.entity(*actor)
                            .insert(MovementTarget {
                                // Project this to zero for now.
                                target: Vec3::new(pos.x, 0.4, pos.z),
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

/// Defines how an agent should look for a water-source.
pub(crate) fn find_drink(
    mut cmd: Commands,
    q: Query<&GlobalTransform, With<DrinkAbility>>,
    water_sources: Query<(Entity, &GlobalTransform), With<WaterSource>>,
    mut actions: Query<(&Actor, &mut ActionState, &ActionSpan), With<FindDrinkAction>>,
) {
    for (Actor(actor), mut state, _) in &mut actions {
        match *state {
            ActionState::Requested => *state = ActionState::Executing,
            ActionState::Executing => {
                info!("Looking for water");
                if let Ok(transform) = q.get(*actor) {
                    // get the food source closes to the agent's current location
                    if let Some((water_source, water_source_pos)) =
                        water_sources.iter().min_by(|(_, ta), (_, tb)| {
                            let a_distance =
                                (ta.translation() - transform.translation()).length_squared();
                            let b_distance =
                                (tb.translation() - transform.translation()).length_squared();
                            a_distance.partial_cmp(&b_distance).unwrap()
                        })
                    {
                        cmd.entity(*actor)
                            .insert(MovementTarget {
                                target: water_source_pos.translation(),
                            })
                            .insert(DrinkTarget {
                                target: water_source,
                            });

                        *state = ActionState::Success;
                    } else {
                        info!("No water sources are closest");
                        *state = ActionState::Cancelled;
                    }
                } else {
                    info!("No entities exist to perform this action");
                    *state = ActionState::Cancelled;
                }
            }
            ActionState::Success => {
                info!("Found water source!");
            }
            ActionState::Cancelled => *state = ActionState::Failure,
            _ => {}
        }
    }
}

/// Defines how an agent should look for a water-source.
pub(crate) fn reproduce_action(
    mut writer: EventWriter<SpawnFauna>,
    mut reproducers: Query<(&mut Reproduction, &GlobalTransform)>,
    mut actions: Query<(&Actor, &mut ActionState, &ActionSpan), With<ReproduceAction>>,
    settings: Res<TileSettings>,
) {
    for (Actor(actor), mut state, _) in &mut actions {
        match *state {
            ActionState::Requested => *state = ActionState::Executing,
            ActionState::Executing => {
                info!("Reproducing");
                if let Ok((mut reproducer, transform)) = reproducers.get_mut(*actor) {
                    if reproducer.value >= 100.0 {
                        info!("SUCESS!");
                        *state = ActionState::Success;
                        reproducer.value = 0.0;
                        let spawn_pos = world_to_pos(&transform.translation(), &settings);
                        writer.send(SpawnFauna {
                            position: Some(spawn_pos),
                        })
                    } else {
                        *state = ActionState::Cancelled;
                    }
                } else {
                    info!("No reproducer to perform this action.");
                    *state = ActionState::Cancelled;
                }
            }
            ActionState::Cancelled => {
                info!("Reproduction cancelled.");
                *state = ActionState::Failure;
            }
            ActionState::Success => {
                info!("Sucessfully reproduced!");
            }
            _ => {}
        }
    }
}
