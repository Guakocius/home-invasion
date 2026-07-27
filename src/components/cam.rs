//! This module defines the core functionality of the Camera used by the Player.

use bevy::prelude::*;
use bevy_camera_controller::free_camera::FreeCamera;

/// Plugin for adding the camera's related systems.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use home_invasion::components::cam::CamPlugin;
///
/// App::new().add_plugins((MinimalPlugins, CamPlugin)).update();
/// ```
pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cam_light);
    }
}

fn setup_cam_light(mut cmds: Commands) {
    cmds.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(10., 10., 10.).looking_at(Vec3::ONE, Vec3::Y),
    ));

    cmds.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 10., 0.).looking_at(vec3(1., 0., 0.), Vec3::Y),
        FreeCamera { ..default() },
    ));
}

#[cfg(test)]
mod tests {}
