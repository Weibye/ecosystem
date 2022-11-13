use bevy::prelude::{default, Camera3dBundle, Commands, Plugin, Transform, Vec3};
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle};

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DefaultPickingPlugins)
            .add_startup_system(spawn_player);
    }
}

fn spawn_player(mut cmd: Commands) {
    // Spawn camera
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PickingCameraBundle::default(),
    ));
}
