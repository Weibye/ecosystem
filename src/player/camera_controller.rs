use std::{f32::consts::PI, ops::Range};

use bevy::prelude::{
    warn, App, Component, Entity, IntoSystemDescriptor, Plugin, Quat, Query, Res, Resource,
    Transform, Vec3, With, Without,
};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin},
    Actionlike,
};

use crate::utils::{lerp, lerp_range, project_to_plane};

#[derive(Component, Debug)]
pub(crate) struct CameraController {
    /// Current camera target.
    target: Entity,
    /// Zoom level of this camera controller.
    zoom: f32, // 0..1
    /// Rotation in radians around the target.
    rotation: f32, // 0..2PI
}

impl CameraController {
    pub(crate) fn new(target: Entity) -> Self {
        Self {
            target,
            zoom: 0.5,
            rotation: 0.0,
        }
    }

    fn set_zoom(&mut self, value: f32) {
        self.zoom = (value).clamp(0.0, 1.0);
    }

    fn add_zoom(&mut self, value: f32) {
        self.set_zoom(self.zoom + value);
    }

    fn set_rot(&mut self, value: f32) {
        let value = value.rem_euclid(2.0 * PI);
        self.rotation = (value).clamp(0.0, 2.0 * PI);
    }

    fn add_rot(&mut self, value: f32) {
        self.set_rot(self.rotation + value);
    }
}

pub(crate) struct CameraControllerPlugin {
    pub(crate) settings: CameraControllerSettings,
}

#[derive(Resource, Clone)]
pub(crate) struct CameraControllerSettings {
    pub(crate) translation_speed: f32,
    pub(crate) rotation_speed: f32,
    pub(crate) zoom_speed: f32,
    pub(crate) zoom: Range<f32>,
}
impl Default for CameraControllerSettings {
    fn default() -> Self {
        Self {
            translation_speed: 0.1,
            rotation_speed: 0.05,
            zoom_speed: 0.04,
            zoom: 2.0..30.0,
        }
    }
}

/// Marker component that the camera should always be looking at.
///
/// TODO: This should have some visible VFX to make sure the player understand
/// what point is being rotated around.
#[derive(Component)]
pub(crate) struct CameraTarget;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone())
            // This plugin maps inputs to an input-type agnostic action-state
            // We need to provide it with an enum which stores the possible actions a player could take
            .add_plugin(InputManagerPlugin::<CameraMovement>::default())
            .add_system(update_camera_input)
            .add_system(update_camera_position.after(update_camera_input));
    }
}

#[derive(Actionlike, Clone, Debug)]
pub(crate) enum CameraMovement {
    Move,
    Zoom,
    Rotate,
}

fn update_camera_input(
    mut cameras: Query<
        (
            &mut CameraController,
            &ActionState<CameraMovement>,
            &Transform,
        ),
        Without<CameraTarget>,
    >,
    mut targets: Query<&mut Transform, (With<CameraTarget>, Without<CameraController>)>,
    settings: Res<CameraControllerSettings>,
) {
    for (mut controller, action, transform) in &mut cameras {
        if let Ok(mut target_transform) = targets.get_mut(controller.target) {
            if action.pressed(CameraMovement::Move) {
                // Move target
                let axis_pair = action.axis_pair(CameraMovement::Move).unwrap();
                let forward = project_to_plane(transform.forward(), Vec3::Y).normalize();
                let right = project_to_plane(transform.right(), Vec3::Y).normalize();

                let delta = forward * axis_pair.y() + right * axis_pair.x();

                target_transform.translation += delta.normalize() * settings.translation_speed;
            }
            if action.pressed(CameraMovement::Rotate) {
                // Rotate camera around a target
                controller.add_rot(action.value(CameraMovement::Rotate) * settings.rotation_speed);
            }
            if action.pressed(CameraMovement::Zoom) {
                // Zoom in and out from a target
                controller
                    .add_zoom(action.value(CameraMovement::Zoom) * settings.zoom_speed * -1.0);
            }
        } else {
            warn!("CameraController does not have a valid camera-target component");
        }
    }
}

fn update_camera_position(
    mut cameras: Query<(&mut Transform, &CameraController), Without<CameraTarget>>,
    targets: Query<&Transform, (With<CameraTarget>, Without<CameraController>)>,
    settings: Res<CameraControllerSettings>,
) {
    for (mut transform, controller) in &mut cameras {
        if let Ok(target_transform) = targets.get(controller.target) {
            let zoom_amount = lerp_range(controller.zoom, &settings.zoom);
            let horizontal_rotation = Quat::from_axis_angle(Vec3::Y, controller.rotation);
            let vertical_rotation =
                Quat::from_axis_angle(Vec3::X, lerp(controller.zoom, 0.1 * PI, 0.3 * PI));

            transform.translation = target_transform.translation
                + horizontal_rotation * vertical_rotation * -Vec3::Z * zoom_amount;
            transform.look_at(target_transform.translation, Vec3::Y);
        }
    }
}
