use bevy::{
    prelude::{
        info, Commands, Component, Entity, EventWriter, GlobalTransform, Query, Res, Transform,
        With,
    },
    time::Time,
};
use big_brain::{
    prelude::ActionState,
    thinker::{ActionSpan, Actor},
};
use bracket_pathfinding::prelude::{a_star_search, Algorithm2D};

use crate::{
    fauna::{
        needs::{Hunger, Reproduction, Thirst},
        SpawnFauna,
    },
    map::{
        tiles::{world_to_pos, TilePos},
        Map,
    },
    resource::{FoodSource, WaterSource},
};

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

/// Component that contians the path to follow.
#[derive(Component, Debug)]
pub(crate) struct MovementPath {
    pub(crate) path: Vec<usize>,
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
// TODO: Move through waypoints
pub(crate) fn move_to_target(
    mut cmd: Commands,
    time: Res<Time>,
    mut agents: Query<(&mut Transform, &TilePos, &mut MovementPath, &MoveAbility)>,
    mut actions: Query<(&Actor, &mut ActionState, &ActionSpan), With<MoveAction>>,
    map: Res<Map>,
) {
    for (Actor(actor), mut state, _) in &mut actions {
        match *state {
            ActionState::Requested => *state = ActionState::Executing,
            ActionState::Cancelled => *state = ActionState::Failure,
            ActionState::Executing => {
                info!("Moving to target");
                if let Ok((mut transform, _, mut path, ability)) = agents.get_mut(*actor) {
                    let mut available_movement = time.delta_seconds() * ability.speed;

                    while available_movement > 0.0 && !path.path.is_empty() {
                        let delta = map.index_to_world(path.path[0]) - transform.translation;

                        if delta.length() > available_movement {
                            transform.translation += delta.normalize() * available_movement;
                            available_movement = 0.0;
                        } else {
                            transform.translation += delta;
                            available_movement -= delta.length();
                            path.path.remove(0);
                        }
                    }
                    if path.path.is_empty() {
                        info!("We arrive at the end of the path!");
                        *state = ActionState::Success;
                    }
                } else {
                    info!("No entities exist to perform this action");
                    *state = ActionState::Cancelled;
                }
            }
            ActionState::Success => {
                info!("Target reached");
                cmd.entity(*actor).remove::<MovementPath>();
            }
            _ => {}
        }
    }
}

/// Defines how an agent should look for a food source.
// TODO: Make this generic
pub(crate) fn find_food(
    mut cmd: Commands,
    agents: Query<(&GlobalTransform, &TilePos), With<EatAbility>>,
    food_sources: Query<(Entity, &GlobalTransform, &TilePos), With<FoodSource>>,
    mut actions: Query<(&Actor, &mut ActionState, &ActionSpan), With<FindFoodAction>>,
    map: Res<Map>,
) {
    for (Actor(actor), mut state, _) in &mut actions {
        match *state {
            ActionState::Requested => *state = ActionState::Executing,
            ActionState::Executing => {
                info!("Looking for food");
                if let Ok((agent_transform, agent_pos)) = agents.get(*actor) {
                    // get the food source closes to the agent's current location
                    if let Some((source_entity, _, source_pos)) =
                        food_sources.iter().min_by(|(_, ta, _), (_, tb, _)| {
                            let a_distance =
                                (ta.translation() - agent_transform.translation()).length_squared();
                            let b_distance =
                                (tb.translation() - agent_transform.translation()).length_squared();
                            a_distance.partial_cmp(&b_distance).unwrap()
                        })
                    {
                        // Find the path
                        let path = a_star_search(
                            map.point2d_to_index(agent_pos.pos.into()),
                            map.point2d_to_index(source_pos.pos.into()),
                            &*map,
                        );

                        if path.success {
                            cmd.entity(*actor)
                                .insert(MovementPath {
                                    // Project this to zero for now.
                                    path: path.steps,
                                })
                                .insert(EatTarget {
                                    target: source_entity,
                                });

                            *state = ActionState::Success;
                        } else {
                            info!("Unable to find a valid path to the food-source");
                            *state = ActionState::Cancelled;
                        }
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
///
/// TODO: This should be defined based on observing ability and discovered knowledge.
/// If the entity remembers a water source in a good location and not have to explore for a new one,
/// that should be used instead.
pub(crate) fn find_drink(
    mut cmd: Commands,
    agents: Query<(&GlobalTransform, &TilePos), With<DrinkAbility>>,
    water_sources: Query<(Entity, &GlobalTransform, &TilePos), With<WaterSource>>,
    mut actions: Query<(&Actor, &mut ActionState, &ActionSpan), With<FindDrinkAction>>,
    map: Res<Map>,
) {
    for (Actor(actor), mut state, _) in &mut actions {
        match *state {
            ActionState::Requested => *state = ActionState::Executing,
            ActionState::Executing => {
                info!("Looking for water");
                if let Ok((agent_transform, agent_pos)) = agents.get(*actor) {
                    // get the food source closes to the agent's current location
                    if let Some((source_entity, _, source_pos)) =
                        water_sources.iter().min_by(|(_, ta, _), (_, tb, _)| {
                            let a_distance =
                                (ta.translation() - agent_transform.translation()).length_squared();
                            let b_distance =
                                (tb.translation() - agent_transform.translation()).length_squared();
                            a_distance.partial_cmp(&b_distance).unwrap()
                        })
                    {
                        // Find the path
                        let path = a_star_search(
                            map.point2d_to_index(agent_pos.pos.into()),
                            map.point2d_to_index(source_pos.pos.into()),
                            &*map,
                        );

                        if path.success {
                            cmd.entity(*actor)
                                .insert(MovementPath {
                                    // Project this to zero for now.
                                    path: path.steps,
                                })
                                .insert(DrinkTarget {
                                    target: source_entity,
                                });

                            *state = ActionState::Success;
                        } else {
                            info!("Unable to find a valid path to the food-source");
                            *state = ActionState::Cancelled;
                        }
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
    map: Res<Map>,
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
                        let spawn_pos = world_to_pos(&transform.translation(), &map.settings);
                        writer.send(SpawnFauna {
                            position: Some(TilePos::from_vec2(spawn_pos)),
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
