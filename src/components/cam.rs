//! This module defines the core functionality of the Camera used by the Player.

use std::f32::consts::FRAC_PI_2;

use super::{game_menu::GameState, player::Player};
use bevy::{
    camera::Viewport,
    camera::visibility::RenderLayers,
    color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

/// Plugin for adding the camera's related systems.
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use home_invasion::components::cam::CamPlugin;
///
/// App::new().add_plugins((MinimalPlugins, CamPlugin)).update();
/// ```
pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (setup_cam, setup_cam_light, grab_cursor, spawn_minimap),
        );
    }
}

/// The Camera Sensitivity. Added for accessibility and configuration reasons.
///
/// # Examples
///
/// ```no_run
/// use bevy::{input::InputPlugin, prelude::*};
/// use home_invasion::components::{
///     player::Player,
///     cam::CameraSensitivity
/// };
///
/// fn spawn_camera(mut cmds: Commands) {
///     cmds.spawn((
///         Player,
///         CameraSensitivity::default(),
///         Transform::from_xyz(0.0, 7.5, 0.0)
///             .looking_at(vec3(5.0, 7.5, 0.0), Vec3::Y),
///         Visibility::default(),
///     ));
/// }
///
/// App::new()
///     .add_plugins((
///         MinimalPlugins,
///         InputPlugin,
///         AssetPlugin::default(),
///     ))
///     .add_systems(Startup, spawn_camera).update();
///
/// ```
#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}

#[derive(Debug, Component)]
struct PlayerCamera;

fn grab_cursor(mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursor_options.single_mut() {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }
}

fn setup_cam(mut cmds: Commands) {
    cmds.spawn((
        Player,
        CameraSensitivity::default(),
        Transform::from_xyz(25.0, 5.5, -25.0).looking_at(vec3(30.0, 7.5, -30.0), Vec3::Y),
        Visibility::default(),
        children![(
            PlayerCamera,
            Camera3d::default(),
            Projection::from(PerspectiveProjection {
                fov: 90.0_f32.to_radians(),
                ..default()
            }),
        ),],
    ));
}

fn setup_cam_light(mut _cmds: Commands) {
    // cmds.spawn((
    //     DirectionalLight::default(),
    //     Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ONE, Vec3::Y),
    //     RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    // ));
}

fn spawn_minimap(mut cmds: Commands) {
    cmds.spawn((
        Camera3d::default(),
        Camera {
            order: 2,
            clear_color: ClearColorConfig::None,
            viewport: Some(Viewport {
                physical_position: UVec2::ZERO,
                physical_size: UVec2::new(150, 150),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 178.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cam_plugin_build() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, CamPlugin)).update();

        assert!(app.is_plugin_added::<CamPlugin>());
    }
}
