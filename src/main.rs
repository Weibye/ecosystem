use agent::{
    actions::{Eat, MoveToFood},
    scorers::Hungry,
    AgentPlugin,
};
use bevy::prelude::*;
use big_brain::prelude::*;
use random::{RandomPlugin, Random};
use resource::ResourcePlugin;
use utils::{get_rand_point_on_board};

mod agent;
mod resource;
mod utils;
mod random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AgentPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(RandomPlugin)
        .insert_resource(Board(Vec2::new(10.0, 10.0)))
        .add_startup_system(setup)
        .add_startup_system(spawn_agent)
        .add_system(hunger_decay)
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
    mut rng: ResMut<Random>
) {
    let height = 0.4;
    let point = get_rand_point_on_board(&mut rng.0, &board);

    let move_and_eat = Steps::build()
        .label("MoveAndEat")
        .step(MoveToFood { speed: 1.0 })
        .step(Eat { per_second: 20.0 });

    let thinker = Thinker::build()
        .label("HungryThinker")
        .picker(FirstToScore { threshold: 0.8 })
        .when(Hungry, move_and_eat);

    // spawn the agent randomly on the board
    cmd.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.2,
            depth: height,
            ..default()
        })),
        material: materials.add(Color::rgb(0.7, 0.3, 0.3).into()),
        transform: Transform::from_xyz(point.x, height, point.y),
        ..default()
    })
    .insert(Hunger {
        per_second: 2.0,
        value: 75.0,
    })
    .insert(thinker);

    // with needs
}

// Resources: Food + Water
// Keep track of how many resources there is of any given type.
// When one is removed, a new one should appear

// NEEDS

#[derive(Component, Debug, Copy, Clone)]
struct Hunger {
    /// How fast the entity gets hungry.
    pub per_second: f32,
    /// Current value of the hunger.
    pub value: f32,
}

/// System that decays all agents' hunger over time.
fn hunger_decay(time: Res<Time>, mut q: Query<&mut Hunger>) {
    for mut hunger in &mut q {
        hunger.value += hunger.per_second * time.delta_seconds();

        if hunger.value >= 100.0 {
            hunger.value = 100.0;
        }
    }
}
