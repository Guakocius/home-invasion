//! A module for specifying the player's core behavior.

use std::f32::consts::FRAC_PI_2;

use super::cam::CameraSensitivity;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

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

/// The Plugin for the [Player's](Player) core functionalities.
///
/// # Examples
///
/// ```
/// use bevy::{input::InputPlugin, prelude::*};
/// use home_invasion::components::player::PlayerPlugin;
///
/// App::new()
///     .add_plugins((
///         MinimalPlugins,
///         InputPlugin,
///         AssetPlugin::default(),
///         PlayerPlugin
///     ))
///     .update();
/// ```
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

fn move_player(
    time: Res<Time>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;
    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        let pitch_limit = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-pitch_limit, pitch_limit);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }

    let mut direction = Vec3::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        let movement_speed = 5.0;

        let normalized_dir = direction.normalize();
        let move_vector = transform.rotation * normalized_dir * Vec3::new(1.0, 0.0, 1.0);

        transform.translation += move_vector * movement_speed * time.delta_secs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::InputPlugin;

    #[test]
    fn test_player_plugin_build() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            InputPlugin,
            AssetPlugin::default(),
            PlayerPlugin,
        ))
        .update();
        assert!(app.is_plugin_added::<PlayerPlugin>());
    }
}
