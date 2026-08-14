//! This module defines the core functionality of the Camera used by the Player.

use std::f32::consts::FRAC_PI_2;

use super::player::Player;
use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

const DEFAULT_RENDER_LAYER: usize = 0;
const VIEW_MODEL_RENDER_LAYER: usize = 1;

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
        app.add_systems(Startup, (spawn_view_model, setup_cam_light, grab_cursor));
    }
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}

#[derive(Debug, Component)]
struct WorldModelCamera;

fn grab_cursor(mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursor_options.single_mut() {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }
}

fn spawn_view_model(mut cmds: Commands) {
    cmds.spawn((
        Player,
        CameraSensitivity::default(),
        Transform::from_xyz(0.0, 7.5, 0.0).looking_at(vec3(5.0, 7.5, 0.0), Vec3::Y),
        Visibility::default(),
        children![
            (
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ),
            (
                Camera3d::default(),
                Camera {
                    order: 1,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ),
        ],
    ));
}

fn setup_cam_light(mut cmds: Commands) {
    cmds.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ONE, Vec3::Y),
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
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
