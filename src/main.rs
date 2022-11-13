use agent::{
    actions::{
        DrinkAbility, DrinkAction, EatAbility, EatAction, FindDrinkAction, FindFoodAction,
        MoveAbility, MoveAction, ReproduceAction,
    },
    needs::{Health, Hunger, Reproduction, Thirst},
    scorers::{Hungry, ReproductionScore, Thirsty},
    AgentPlugin,
};
use bevy::prelude::*;
use big_brain::prelude::*;
use random::{Random, RandomPlugin};
use resource::ResourcePlugin;
use utils::get_rand_point_on_board;

mod agent;
mod random;
mod resource;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AgentPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(RandomPlugin)
        .insert_resource(Board(Vec2::new(10.0, 10.0)))
        .add_startup_system(setup)
        .add_startup_system(spawn_agent)
        .run();
}

struct Board(pub Vec2);

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera
    cmd.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn ground
    cmd.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.2, 1.0, 0.3).into()),
        ..default()
    });

    // ambient light
    cmd.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.04,
    });

    // Spawn light
    cmd.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_agent(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board: Res<Board>,
    mut rng: ResMut<Random>,
) {
    let height = 0.4;
    let point = get_rand_point_on_board(&mut rng.0, &board);

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
    cmd.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.2,
            depth: height,
            ..default()
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.5).into()),
        transform: Transform::from_xyz(point.x, height, point.y),
        ..default()
    })
    .insert(Hunger {
        per_second: 1.0,
        value: 75.0,
    })
    .insert(Thirst {
        per_second: 3.0,
        value: 50.0,
    })
    .insert(Reproduction { value: 50.0 })
    .insert(Health { value: 80.0 })
    .insert(EatAbility { speed: 80.0 })
    .insert(DrinkAbility { speed: 80.0 })
    .insert(MoveAbility { speed: 5.0 })
    .insert(thinker);

    // with needs
}
