use bevy::prelude::*;
use bevy_camera_controller::free_camera::FreeCamera;

#[derive(Component)]
pub struct Player {
    pub is_dead: bool,
    pub radius: f32,
    pub length: f32,
    pub pos: Vec3,
    pub free_camera: Option<FreeCamera>,
    pub camera: Option<Camera>,
}

impl Player {
    fn new(
        radius: f32,
        length: f32,
        free_camera: Option<FreeCamera>,
        camera: Option<Camera>,
    ) -> Self {
        Self {
            is_dead: false,
            radius,
            length,
            pos: Vec3::ZERO,
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
    let player = Player::new(0.5, 1.0, camera, None);
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
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
