//! A module for specifying the player's core behavior.

use std::f32::consts::FRAC_PI_2;

use super::cam::CameraSensitivity;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use bevy_camera_controller::free_camera::FreeCamera;

/// The Player [`Component`].
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use home_invasion::components::player::Player;
///
/// ```
#[derive(Debug, Component)]
pub struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    //let mesh = meshes.add(Capsule3d::new(player.radius, player.length / 2.0));

    /*commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::BLACK,
            ..default()
        })),
        Transform::from_translation(player.pos),
        player,
    ));
    commands.insert_resource(player_speed);*/
}

/// The Plugin for the [Player's](Player) core functionalities.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use home_invasion::components::player::PlayerPlugin;
///
/// App::new()
///     .add_plugins((MinimalPlugins, AssetPlugin::default(),
/// PlayerPlugin))
///     .init_asset::<Mesh>()
///     .init_asset::<StandardMaterial>()
///     .update();
/// ```
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

fn move_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_plugin_build() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), PlayerPlugin))
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .update();
        assert!(app.is_plugin_added::<PlayerPlugin>());
    }
}
