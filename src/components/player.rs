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

#[derive(Component)]
pub struct Player {
    pub is_dead: bool,
    pub radius: f32,
    pub length: f32,
    pub pos: Vec3,
    pub speed: PlayerSpeed,
    pub free_camera: Option<FreeCamera>,
    pub camera: Option<Camera>,
}

impl Player {
    fn new(
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

/// The Plugin for the [Players](home_invasion::components::player::Player) core functionalities.
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
mod tests {}
