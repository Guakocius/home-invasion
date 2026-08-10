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
    /// Creates a new Wall with a specified width, depth, a fix height, a position and its texture.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use home_invasion::components::rooms::Wall;
    ///
    /// const ERROR_MARGIN: f32 = 0.1;
    ///
    /// let wall = Wall::new(10.0, 2.0, Vec3::ZERO, String::from("assets/textures/test.png"));
    ///
    /// assert!((wall.height - 30.0).abs() < ERROR_MARGIN);
    /// assert!((wall.width - 10.0).abs() < ERROR_MARGIN);
    /// assert!((wall.depth - 2.0).abs() < ERROR_MARGIN);
    /// ```
    #[must_use]
    pub fn new(width: f32, depth: f32, pos: Vec3, texture: String) -> Self {
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

impl Door {
    fn new(position: Vec3) -> Self {
        Self {
            height: 1.9,
            width: 0.914,
            depth: 0.14,
            pos: position,
            texture: "NONE".into(),
        }
    }
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
    fn build(&self, _app: &mut App) {
        // app.add_systems(Startup, generate_rooms);
    }
}

/// Generate rooms based on the given parameters.
///
/// # Examples
///
/// ```
/// use home_invasion::components::rooms::generate_rooms;
///
/// let room = generate_rooms(16.0, 32.0, 16.0);
/// ```
#[must_use]
pub fn generate_rooms(half_width: f32, half_depth: f32, step: f32) -> Vec<Vec3> {
    let mut positions = Vec::new();

    // West wall
    let mut z = -half_depth;
    while z <= half_depth {
        positions.push(Vec3::new(-half_width, 0.0, z));
        z += step;
    }

    // North wall
    let mut x = -half_width + step;
    while x <= half_width {
        positions.push(Vec3::new(x, 0.0, half_depth));
        x += step;
    }

    // East wall
    z = half_depth - step;
    while z >= -half_depth {
        positions.push(Vec3::new(half_width, 0.0, z));
        z -= step;
    }

    // South wall
    x = half_width - step;
    while x > -half_width {
        positions.push(Vec3::new(x, 0.0, -half_depth));
        x -= step;
    }

    positions
}

/// This module defines the Office.
pub mod office {
    use std::{collections::HashMap, f32::consts::PI};

    use crate::components::rooms::Door;

    use super::{Rooms, generate_rooms};
    use bevy::{
        animation::{AnimationPlayer, graph::AnimationGraph},
        color::palettes::css::WHITE,
        ecs::event::Trigger,
        gltf::{GltfExtras, GltfMaterialExtras, GltfMaterialName, GltfMeshExtras, GltfSceneExtras},
        input::common_conditions::input_just_pressed,
        prelude::*,
        world_serialization::WorldInstanceReady,
    };

    const SCALE: Vec3 = Vec3::new(4.0, 4.0, 4.0);

    /// Plugin for all the systems associated with the [`Rooms::Office`].
    ///
    /// # Examples
    /// Loads and spawns the needed x.blend files.
    /// ```no_run
    /// use bevy::prelude::*;
    /// use home_invasion::components::rooms::office::OfficePlugin;
    ///
    /// App::new().add_plugins((MinimalPlugins, OfficePlugin)).update();
    /// ```
    pub struct OfficePlugin;

    impl Plugin for OfficePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(
                Startup,
                (setup_office, spawn_bookshelf, spawn_table, setup_animation),
            )
            .add_systems(Update, open_door.run_if(input_just_pressed(KeyCode::KeyE)));
        }
    }

    fn setup_office(mut cmds: Commands, asset_server: Res<AssetServer>) {
        let positions = generate_rooms(16.0, 8.0, 8.0);

        let assets: [String; 2] = [
            String::from("Wall_office"),
            String::from("Wall_corner_1_office"),
        ];

        let walls = [1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
            .iter()
            .map(|i| assets[*i].clone())
            .collect::<Vec<String>>();

        let len = positions.len();

        for idx in 0..len {
            let curr_pos = positions[idx];

            let mut transform =
                Transform::from_translation(Vec3::new(curr_pos[0], curr_pos[1], curr_pos[2]));

            match idx {
                1 | 5 => continue,
                2..=4 => transform.rotate_local_y(PI / 2.0),
                6 | 7 => transform.rotate_local_y(PI),
                8..=11 => transform.rotate_local_y(-PI / 2.0),
                _ => {}
            }

            cmds.spawn((
                WorldAssetRoot(asset_server.load(
                    GltfAssetLabel::Scene(0).from_asset(format!("models/{:}.glb", walls[idx])),
                )),
                transform.with_scale(SCALE),
            ));
        }
    }

    #[derive(Component)]
    struct Bookshelf;

    fn spawn_bookshelf(
        mut cmds: Commands,
        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let bookshelf_pos = [-6.0, -3.0, 0.0, 3.0, 6.0];

        for pos in &bookshelf_pos {
            let mut transform = Transform::from_translation(Vec3::new(15.0, 0.0, *pos));
            transform.rotate_local_y(PI / 2.0);
            cmds.spawn((
                WorldAssetRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("models/Office_Bookshelf.glb")),
                ),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::from(WHITE),
                    unlit: true,
                    ..default()
                })),
                transform.with_scale(SCALE),
                Bookshelf,
            ))
            .observe(apply_bookshelf_texture);
        }
    }

    fn apply_bookshelf_texture(
        scene_ready: On<WorldInstanceReady>,
        mut cmds: Commands,
        asset_server: Res<AssetServer>,
        children: Query<&Children>,
        bookshelf_query: Query<&Bookshelf>,
        mesh_materials: Query<(&MeshMaterial3d<StandardMaterial>, &GltfMaterialName)>,
        mut standard_mat: ResMut<Assets<StandardMaterial>>,
    ) {
        let Ok(_bookshelf_query) = bookshelf_query.get(scene_ready.entity) else {
            // info!("{} doesn't have Component: Bookshelf", scene_ready.entity);
            return;
        };

        for descendant in children.iter_descendants(scene_ready.entity) {
            let Ok((_id, material_name)) = mesh_materials.get(descendant) else {
                continue;
            };

            let texture: Handle<Image> = asset_server.load("textures/Dark_Wood_texture.png");
            let material: Handle<StandardMaterial> = standard_mat.add(StandardMaterial {
                base_color_texture: Some(texture),
                ..default()
            });

            match material_name.0.as_str() {
                "Wooden" => {
                    cmds.entity(descendant)
                        .insert(MeshMaterial3d(material.clone()));
                }
                name => info!("Not replacing: {name}"),
            }
        }
    }

    fn spawn_table(
        mut cmds: Commands,
        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let bookshelf_pos = Vec3::new(5.0, 0.0, -0.5);

        let mut transform = Transform::from_translation(bookshelf_pos);
        transform.rotate_local_y(PI);
        cmds.spawn((
            WorldAssetRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Office_Table.glb")),
            ),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(WHITE),
                unlit: true,
                ..default()
            })),
            transform.with_scale(SCALE),
            Bookshelf,
        ));
    }

    #[derive(Component, Clone)]
    struct Animations {
        handle: Handle<AnimationGraph>,
        index: HashMap<String, AnimationNodeIndex>,
    }

    fn setup_animation(
        mut cmds: Commands,
        asset_server: Res<AssetServer>,
        mut graphs: ResMut<Assets<AnimationGraph>>,
    ) {
        let door_path = "models/Wall_office_door.glb";

        let mut graph = AnimationGraph::new();
        let mut hash = HashMap::new();

        for (i, name) in ["Door", "Door_Handles", "Door_inner_glas"]
            .iter()
            .enumerate()
        {
            let clip = asset_server.load(GltfAssetLabel::Animation(i).from_asset(door_path));
            let idx = graph.add_clip(clip, 1.0, graph.root);
            hash.insert(name.to_string(), idx);
        }

        let graph_handle = graphs.add(graph);

        let animations = Animations {
            handle: graph_handle,
            index: hash,
        };

        let scene = WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Wall_office_door.glb")),
        );

        let pos = generate_rooms(16.0, 8.0, 8.0);

        for i in &[1, 5] {
            let curr_pos = pos[*i];
            let mut transform = Transform::from_xyz(curr_pos.x, curr_pos.y, curr_pos.z);
            if *i == 5 {
                transform.rotate_local_y(PI / 2.0);
            };

            let door = Door::new(transform.translation);

            cmds.spawn((
                scene.clone(),
                animations.clone(),
                transform.with_scale(SCALE),
                door,
            ))
            .observe(door_animation_ready);
        }
    }

    fn door_animation_ready(
        scene_ready: On<WorldInstanceReady>,
        mut cmds: Commands,
        children: Query<&Children>,
        animations: Query<&Animations>,
        players: Query<&AnimationPlayer>,
    ) {
        let Ok(animations) = animations.get(scene_ready.entity) else {
            return;
        };

        for child in children.iter_descendants(scene_ready.entity) {
            if players.get(child).is_ok() {
                cmds.entity(child)
                    .insert(AnimationGraphHandle(animations.handle.clone()))
                    .insert(animations.clone());
            }
        }
    }

    fn open_door(
        mut door_query: Query<(Entity, &GlobalTransform, &mut AnimationPlayer, &Animations)>,
        cam_query: Single<(&Camera3d, &GlobalTransform)>,
    ) {
        let (_cam, cam_transform) = cam_query.into_inner();

        for (_door_entity, door_transform, mut player, animations) in &mut door_query.iter_mut() {
            if cam_transform
                .translation()
                .distance(door_transform.translation())
                > 10.0
            {
                info!(
                    "CAM TOO FAR AWAY From Door: {:}",
                    cam_transform
                        .translation()
                        .distance(door_transform.translation())
                );
                continue;
            }

            let Some(&door_idx) = animations.index.get("Door") else {
                continue;
            };

            play_animation(door_idx, &mut player, animations)
        }
    }

    fn play_animation(
        door_idx: AnimationNodeIndex,
        player: &mut AnimationPlayer,
        animations: &Animations,
    ) {
        if let Some(action) = &mut player.animation(door_idx) {
            if action.is_finished() {
                info!("Replaying door animation..");
                player.play(door_idx).replay();

                if let Some(&handle_idx) = animations.index.get("Door_Handles") {
                    player.play(handle_idx).replay();
                }
            }
        } else {
            info!("Is currently Playing");
        }
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
