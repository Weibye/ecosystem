use bevy::prelude::{
    default, shape, Assets, Color, Commands, Entity, EventReader, IntoSystemDescriptor, Mesh,
    PbrBundle, Plugin, Res, ResMut, StandardMaterial, Transform,
};
use bevy_mod_picking::PickableBundle;
use bevy_turborand::{DelegatedRng, GlobalRng};
use big_brain::{
    prelude::{FirstToScore, Steps},
    thinker::Thinker,
};

use crate::{
    agent::{
        actions::{
            DrinkAbility, DrinkAction, EatAbility, EatAction, FindDrinkAction, FindFoodAction,
            MoveAbility, MoveAction, ReproduceAction,
        },
        scorers::{Hungry, ReproductionScore, Thirsty},
        AgentPlugin,
    },
    map::{get_rand_pos, pos_to_world, TilePosition, TileSettings},
    utils::lerp_range,
};

use self::needs::{
    death, health_update, hunger_decay, reproduction_update, thirst_decay, Health, Hunger,
    Reproduction, Thirst,
};

pub(crate) mod needs;

/// This plugin governs the needs of the fauna, as well as
pub(crate) struct FaunaPlugin;

impl Plugin for FaunaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(AgentPlugin)
            .add_event::<SpawnFauna>()
            .add_event::<DespawnFauna>()
            .add_system(hunger_decay.before(health_update))
            .add_system(thirst_decay.before(health_update))
            .add_system(health_update.before(reproduction_update))
            .add_system(reproduction_update)
            .add_system(death.after(health_update))
            .add_system(despawn_agent.after(death))
            .add_system(spawn_agent);
    }
}

/// Events that spawns one unit of fauna
pub(crate) struct SpawnFauna {
    /// Position to spawn. If none, will use random position on the board.
    // TODO: When spawning due to reproduction, the new Fauna should spawn on a free tile next to the parent.
    pub(crate) position: Option<TilePosition>,
}

pub(crate) struct DespawnFauna {
    entity: Entity,
}

/// System that despawns a fauna-agent when the `DespawnFauna`-event is triggered.
fn despawn_agent(mut cmd: Commands, mut events: EventReader<DespawnFauna>) {
    for event in &mut events.iter() {
        // TODO: A dead fauna should produce some food
        cmd.entity(event.entity).despawn();
    }
}

fn spawn_agent(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalRng>,
    mut events: EventReader<SpawnFauna>,
    settings: Res<TileSettings>,
) {
    for event in &mut events.iter() {
        let height = 0.4;
        let spawn_pos = if event.position.is_some() {
            event.position.unwrap()
        } else {
            get_rand_pos(rng.get_mut(), &settings)
        };

        let spawn_translation = pos_to_world(&spawn_pos, &settings);

        let move_and_eat = Steps::build()
            .label("FindFoodMoveAndEat")
            .step(FindFoodAction)
            .step(MoveAction)
            .step(EatAction);

        let move_and_drink = Steps::build()
            .label("FindDrinkMoveAndEat")
            .step(FindDrinkAction)
            .step(MoveAction)
            .step(DrinkAction);

        let thinker = Thinker::build()
            .label("AgentThinker")
            .picker(FirstToScore { threshold: 0.8 })
            .when(Hungry, move_and_eat)
            .when(Thirsty, move_and_drink)
            .when(ReproductionScore, ReproduceAction);

        // TODO: These ranges should be given by the Fauna archetype
        cmd.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.2,
                    depth: height,
                    ..default()
                })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.5).into()),
                transform: Transform::from_translation(spawn_translation),
                ..default()
            },
            Hunger {
                per_second: lerp_range(rng.f32(), 0.5..3.0),
                value: lerp_range(rng.f32(), 20.0..80.0),
            },
            Thirst {
                per_second: lerp_range(rng.f32(), 0.5..5.0),
                value: lerp_range(rng.f32(), 20.0..80.0),
            },
            Reproduction {
                value: lerp_range(rng.f32(), 20.0..80.0),
            },
            Health {
                value: lerp_range(rng.f32(), 20.0..80.0),
            },
            EatAbility {
                speed: lerp_range(rng.f32(), 20.0..80.0),
            },
            DrinkAbility {
                speed: lerp_range(rng.f32(), 20.0..80.0),
            },
            MoveAbility {
                speed: lerp_range(rng.f32(), 1.5..10.0),
            },
            thinker,
            PickableBundle::default(),
        ));
    }
}
