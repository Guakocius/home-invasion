//! This module defines the room layout and the logic of each of the house's rooms.

use bevy::prelude::*;
use bevy::prelude::{Component, States};
use std::fmt;

#[derive(Component, Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, States)]
/// All rooms of the house with a boolean signifying if whether the player is inside this room or
/// not.
///
/// # Examples
///
/// ```
/// use home_invasion::components::rooms::Rooms;
///
/// let room = Rooms::Basement(true);
/// println!("{room}");
/// ```
pub enum Rooms {
    /// The house's basement.
    Basement(bool),
    /// The house's bathroom.
    Bathroom(bool),
    /// The house's bedroom.
    Bedroom(bool),
    /// The house's hallway.
    Hallway(bool),
    /// The house's home office.
    HomeOffice(bool),
    /// The house's kid's room.
    KidsRoom(bool),
    /// The house's kitchen.
    Kitchen(bool),
    /// The house's living room.
    LivingRoom(bool),
    /// The house's office.
    Office(bool),
    /// The house's shower.
    Shower(bool),
    /// The house's first storage.
    Storage1(bool),
    /// The house's second storage.
    Storage2(bool),
    /// The house's toilet.
    Toilet(bool),
}

/// This structure defines the walls of each [`Room`].
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use home_invasion::components::rooms::Wall;
///
/// const ERROR_MARGIN: f32 = 0.1;
///
/// let wall = Wall { height: 10.0, width: 5.0, depth: 2.0, pos: Vec3::ZERO, texture: "/assets/textures/test.jpg".into()};
///
/// assert!((wall.height - 10.0).abs() < ERROR_MARGIN);
/// assert!((wall.width - 5.0).abs() < ERROR_MARGIN);
/// assert!((wall.depth - 2.0).abs() < ERROR_MARGIN);
/// assert_eq!(wall.pos, Vec3::ZERO);
/// assert_eq!(wall.texture, String::from("/assets/textures/test.jpg"));
/// ```
#[derive(Component)]
pub struct Wall {
    /// The wall's height.
    pub height: f32,
    /// The wall's width.
    pub width: f32,
    /// The wall's depth.
    pub depth: f32,
    /// The wall's 3-dimensional position in the [`World`].
    pub pos: Vec3,
    /// The wall's texture file path.
    pub texture: String,
}

const WALL_HEIGHT: f32 = 30.0;

impl Wall {
    #[must_use]
    fn new(width: f32, depth: f32, pos: Vec3, texture: String) -> Self {
        Self {
            height: WALL_HEIGHT,
            width,
            depth,
            pos,
            texture,
        }
    }
}

/// This structure defines the doors of each [`Room`].
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use home_invasion::components::rooms::Door;
///
/// const ERROR_MARGIN: f32 = 0.1;
///
/// let door = Door { height: 5.0, width: 2.0, depth: 1.0, pos: Vec3::ZERO, texture: "/assets/textures/test.jpg".into()};
///
/// assert!((door.height - 5.0).abs() < ERROR_MARGIN);
/// assert!((door.width - 2.0).abs() < ERROR_MARGIN);
/// assert!((door.depth - 1.0).abs() < ERROR_MARGIN);
/// assert_eq!(door.pos, Vec3::ZERO);
/// assert_eq!(door.texture, String::from("/assets/textures/test.jpg"));
/// ```
#[derive(Component)]
pub struct Door {
    /// The door's height.
    pub height: f32,
    /// The door's width.
    pub width: f32,
    /// The door's depth.
    pub depth: f32,
    /// The door's 3-dimensional position in the [`World`].
    pub pos: Vec3,
    /// The door's texture file path.
    pub texture: String,
}

/// This structure defines each `Room` and its contents.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use home_invasion::components::rooms::{Door, Room, Rooms, Wall};
///
/// let mut walls: Vec<Wall> = Vec::new();
/// const WALL_HEIGHT: f32 = 30.0;
/// walls.push(Wall { height: WALL_HEIGHT, width: 50.0, depth: 5.0, pos: Vec3::new(30.0, 50.0, 5.0), texture: "/assets/textures/test_wall1.jpg".into() });
/// walls.push(Wall { height: WALL_HEIGHT, width: 50.0, depth: 5.0, pos: Vec3::new(60.0, 100.0, 10.0), texture: "/assets/textures/test_wall2.jpg".into() });
/// walls.push(Wall { height: WALL_HEIGHT, width: 50.0, depth: 5.0, pos: Vec3::new(90.0, 150.0, 15.0), texture: "/assets/textures/test_wall3.jpg".into() });
/// walls.push(Wall { height: WALL_HEIGHT, width: 50.0, depth: 5.0, pos: Vec3::new(120.0, 200.0, 20.0), texture: "/assets/textures/test_wall4.jpg".into() });
///
/// let room = Room {
///     room: Rooms::Basement(true),
///     walls,
///     door: Door { height: 10.0, width: 5.0, depth: 1.0, pos: Vec3::ZERO, texture: "/assets/textures/test_door.jpg".into() },
///     pos: Vec3::ZERO,
///     texture: "/assets/textures/test_room.jpg".into(),
/// };
/// ```
#[derive(Component)]
pub struct Room {
    /// The type of Room and if the player is currently inside it.
    pub room: Rooms,
    /// The Room's [Walls](Wall).
    pub walls: Vec<Wall>,
    /// The Room's [Door].
    pub door: Door,
    /// The Room's 3-dimensional position in the [`World`].
    pub pos: Vec3,
    /// The Room's texture file path.
    pub texture: String,
}

impl fmt::Display for Rooms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Rooms::Basement(_) => "Basement",
            Rooms::Bathroom(_) => "Bathroom",
            Rooms::Bedroom(_) => "Bedroom",
            Rooms::Hallway(_) => "Hallway",
            Rooms::HomeOffice(_) => "Home Office",
            Rooms::KidsRoom(_) => "Kid's Room",
            Rooms::Kitchen(_) => "Kitchen",
            Rooms::LivingRoom(_) => "Living Room",
            Rooms::Office(_) => "Office",
            Rooms::Shower(_) => "Shower",
            Rooms::Storage1(_) => "Storage 1",
            Rooms::Storage2(_) => "Storage 2",
            Rooms::Toilet(_) => "Toilet",
        };
        write!(f, "{name}")
    }
}

/// Plugin for all systems associated with the [`Rooms`].
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use home_invasion::components::rooms::RoomsPlugin;
///
/// App::new().add_plugins((MinimalPlugins, RoomsPlugin)).update();
/// ```
pub struct RoomsPlugin;

impl Plugin for RoomsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_rooms)
            .add_plugins(office::OfficePlugin);
    }
}

fn generate_rooms(_commands: Commands) {
    let _wall = Wall::new(50.0, 2.0, Vec3::ZERO, "todo.jpg".into());
}

pub mod office {
    use crate::Rooms;
    use bevy::{
        gltf::{GltfExtras, GltfMaterialExtras, GltfMeshExtras, GltfSceneExtras},
        prelude::*,
    };

    pub struct OfficePlugin;

    impl Plugin for OfficePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, setup);
        }
    }

    fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
        let positions = wall_positions();

        let assets: [String; 4] = [
            String::from("Wall_office"),
            String::from("Wall_corner_1_office"),
            String::from("Wall_corner_2_office"),
            String::from("Wall_office_door"),
        ];

        let walls = vec![1, 0, 0, 0, 1, 0, 1, 3, 0, 0, 1, 3]
            .iter()
            .map(|i| assets[*i].clone())
            .collect::<Vec<String>>();

        let len = positions.len();

        for idx in 0..len {
            let curr_pos = positions[idx];

            let mut transform =
                Transform::from_translation(Vec3::new(curr_pos[0], curr_pos[1], curr_pos[2]));

            match idx {
                0 => {
                    transform = transform.looking_at(vec3(10.0, 0.0, 0.0), Vec3::Y);
                }
                1 | 2 | 3 => {
                    transform = transform.looking_at(vec3(curr_pos[0].clone(), 0.0, 10.0), Vec3::Y);
                }
                4 => {
                    transform = transform.looking_at(
                        vec3(curr_pos[0].clone(), 0.0, curr_pos[2].clone() * 10.0),
                        Vec3::Y,
                    );
                }
                5 => {
                    transform = transform.looking_at(vec3(curr_pos[0].clone(), 0.0, -1.0), Vec3::Y);
                    transform.rotate_local_y(1.57);
                }
                6 => {
                    transform = transform.looking_at(
                        vec3(curr_pos[0].clone(), 0.0, curr_pos[2].clone() * 10.0),
                        Vec3::Y,
                    );
                    transform.rotate_local_y(-1.57);
                }
                11 => {
                    transform = transform.looking_at(vec3(10.0, 0.0, 0.0), Vec3::Y);
                }
                _ => {}
            }

            cmds.spawn((
                WorldAssetRoot(asset_server.load(
                    GltfAssetLabel::Scene(0).from_asset(format!("models/{:}.glb", walls[idx])),
                )),
                transform,
            ));
        }
    }

    fn wall_positions() -> Vec<[f32; 3]> {
        let pos: Vec<[f32; 3]> = vec![
            [2.0, 0.0, 0.0],
            [2.0, 0.0, 1.0],
            [2.0, 0.0, 3.0],
            [2.0, 0.0, 5.0],
            [2.0, 0.0, 7.0],
            [0.0, 0.0, 7.0],
            [-2.0, 0.0, 7.0],
            [-2.0, 0.0, 5.0],
            [-2.0, 0.0, 3.0],
            [-2.0, 0.0, 1.0],
            [-2.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
        ];
        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERROR_MARGIN: f32 = 0.1;

    #[test]
    fn test_rooms_fmt() {
        assert_eq!(format!("{}", Rooms::Basement(true)), "Basement");
        assert_eq!(format!("{}", Rooms::Bathroom(true)), "Bathroom");
        assert_eq!(format!("{}", Rooms::Bedroom(true)), "Bedroom");
        assert_eq!(format!("{}", Rooms::Hallway(true)), "Hallway");
        assert_eq!(format!("{}", Rooms::HomeOffice(true)), "Home Office");
        assert_eq!(format!("{}", Rooms::KidsRoom(true)), "Kid's Room");
        assert_eq!(format!("{}", Rooms::Kitchen(true)), "Kitchen");
        assert_eq!(format!("{}", Rooms::LivingRoom(true)), "Living Room");
        assert_eq!(format!("{}", Rooms::Office(true)), "Office");
        assert_eq!(format!("{}", Rooms::Shower(true)), "Shower");
        assert_eq!(format!("{}", Rooms::Storage1(true)), "Storage 1");
        assert_eq!(format!("{}", Rooms::Storage2(true)), "Storage 2");
        assert_eq!(format!("{}", Rooms::Toilet(true)), "Toilet");
    }

    #[test]
    fn test_wall_generation() {
        let wall = Wall::new(
            10.0,
            2.0,
            Vec3::ZERO,
            "/assets/textures/test_wall1.jpg".into(),
        );

        assert!((wall.height - WALL_HEIGHT) < ERROR_MARGIN);
        assert!((wall.width - 10.0) < ERROR_MARGIN);
        assert!((wall.depth - 2.0) < ERROR_MARGIN);
        assert_eq!(wall.pos, Vec3::ZERO);
    }

    #[test]
    fn test_rooms_plugin_build() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, RoomsPlugin));
        app.update();
        assert!(app.is_plugin_added::<RoomsPlugin>());
    }
}
