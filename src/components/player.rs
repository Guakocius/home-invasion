//! A module for specifying the player's core behavior.

/// The speed of the player defined as a resource for re-using.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use home_invasion::components::player::PlayerSpeed;
///
/// App::new().insert_resource(PlayerSpeed(100.0));
/// ```
#[derive(Resource, Debug, Clone)]
pub struct PlayerSpeed(
    /// Player speed as a 32 bit floating point number.
    pub f32,
);

use bevy::prelude::*;
use bevy_camera_controller::free_camera::FreeCamera;

/// The Player [`Component`].
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_camera_controller::free_camera::FreeCamera;
/// use home_invasion::components::player::{Player, PlayerSpeed};
///
/// let camera = Some(FreeCamera { ..default() });
/// let speed = PlayerSpeed(100.0);
/// let player = Player::new(1.0, 1.0, speed, camera, None);
///
/// assert!(!player.is_dead);
/// assert_eq!(player.radius, 1.0);
/// assert_eq!(player.length, 1.0);
/// assert_eq!(player.pos, Vec3::ZERO);
/// assert!(player.free_camera.is_some());
/// assert!(player.camera.is_none());
/// ```
#[derive(Component)]
pub struct Player {
    /// True if the player is dead, false otherwise.
    pub is_dead: bool,
    /// The Player's radius.
    pub radius: f32,
    /// The Player's length.
    pub length: f32,
    /// The Player's position on the World map.
    pub pos: Vec3,
    /// The Player's speed.
    pub speed: PlayerSpeed,
    /// Option for a [`FreeCamera`] (used for debugging).
    pub free_camera: Option<FreeCamera>,
    /// Option for a normal [`Camera`] (used in production or to see the World from the eyes of the
    /// Player).
    pub camera: Option<Camera>,
}

impl Player {
    #[must_use]
    /// Creates a new Player with given radius, length, [`PlayerSpeed`], either [`FreeCamera`] or [`Camera`] (the other is [`None`]) and some default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_camera_controller::free_camera::FreeCamera;
    /// use home_invasion::components::player::{Player, PlayerSpeed};
    ///
    /// let player = Player::new(1.0, 1.0, PlayerSpeed(100.0), Some(FreeCamera { ..default() }), None);
    ///
    /// assert!(!player.is_dead);
    /// assert_eq!(player.radius, 1.0);
    /// assert_eq!(player.length, 1.0);
    /// assert_eq!(player.pos, Vec3::ZERO);
    /// assert!(player.free_camera.is_some());
    /// assert!(player.camera.is_none());
    /// ```
    pub fn new(
        radius: f32,
        length: f32,
        speed: PlayerSpeed,
        free_camera: Option<FreeCamera>,
        camera: Option<Camera>,
    ) -> Self {
        Self {
            is_dead: false,
            radius,
            length,
            pos: Vec3::ZERO,
            speed,
            free_camera,
            camera,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera = Some(FreeCamera { ..default() });
    let player_speed = PlayerSpeed(100.0);
    let player = Player::new(0.5, 1.0, player_speed.clone(), camera, None);
    let mesh = meshes.add(Capsule3d::new(player.radius, player.length / 2.0));

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::BLACK,
            ..default()
        })),
        Transform::from_translation(player.pos),
        player,
    ));
    commands.insert_resource(player_speed);
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
        app.add_systems(Startup, setup);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_fields() {
        const ERROR_MARGIN: f32 = 0.1;
        let speed = PlayerSpeed(100.0);
        let camera = Some(Camera::default());
        let player = Player::new(1.0, 1.0, speed, None, camera);
        assert!(!player.is_dead);
        assert!((player.radius - 1.0).abs() < ERROR_MARGIN);
        assert!((player.length - 1.0).abs() < ERROR_MARGIN);
        assert_eq!(player.pos, Vec3::ZERO);
        assert!(player.free_camera.is_none());
        assert!(player.camera.is_some());
    }
}
