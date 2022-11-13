use bevy::prelude::{
    default, shape, Assets, Color, Commands, Entity, EventReader, IntoSystemDescriptor, Mesh,
    PbrBundle, Plugin, Res, ResMut, StandardMaterial, Transform, Vec2,
};
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
    utils::get_rand_point_on_board,
    Board,
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
    pub(crate) position: Option<Vec2>,
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
    board: Res<Board>,
    mut rng: ResMut<GlobalRng>,
    mut events: EventReader<SpawnFauna>,
) {
    for event in &mut events.iter() {
        let height = 0.4;

        let spawn_point = if event.position.is_some() {
            event.position.unwrap()
        } else {
            get_rand_point_on_board(&mut *rng.get_mut(), &board)
        };

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

        // spawn the agent randomly on the board
        cmd.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.2,
                    depth: height,
                    ..default()
                })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.5).into()),
                transform: Transform::from_xyz(spawn_point.x, height, spawn_point.y),
                ..default()
            },
            Hunger {
                per_second: 1.0,
                value: 75.0,
            },
            Thirst {
                per_second: 3.0,
                value: 50.0,
            },
            Reproduction { value: 50.0 },
            Health { value: 80.0 },
            EatAbility { speed: 80.0 },
            DrinkAbility { speed: 80.0 },
            MoveAbility { speed: 5.0 },
            thinker,
        ));
    }
}
