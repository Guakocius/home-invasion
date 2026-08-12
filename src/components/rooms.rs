//! This module defines the room layout and the logic of each of the house's rooms.

use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::prelude::{Component, States};
use std::f32::consts::PI;
use std::fmt;

const ERROR_MARGIN: f32 = 0.1;

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
/// ```
/// ```

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Wall;

/// This structure defines the doors of each [`Room`].
///
/// # Examples
///
/// ```
/// ```
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Door;

/// This structure defines each `Room` and its contents.
///
/// # Examples
///
/// ```
/// ```
#[derive(Component, Debug, Clone)]
pub struct Room {
    /// The type of Room.
    pub room_type: Rooms,
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

pub enum WallType {
    Standard,
    Corner,
    Door,
}

pub struct WallSegment {
    pub transform: Transform,
    pub wall_type: WallType,
}

#[derive(Clone)]
pub struct PropSpec {
    pub asset_path: String,
    pub transform: Transform,
    pub texture_path: Option<String>,
}

#[derive(Clone)]
pub struct RoomConfig {
    pub name: String,
    pub half_width: f32,
    pub half_depth: f32,
    pub step: f32,
    pub wall_asset: String,
    pub corner_asset: String,
    pub door_asset: String,
    pub door_indices: Vec<usize>,
    pub props: Vec<PropSpec>,
}

#[derive(Component, Clone)]
pub struct DoorAnimation {
    pub handle: Handle<AnimationGraph>,
    pub node_indices: HashMap<String, AnimationNodeIndex>,
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
pub fn generate_rooms(config: &RoomConfig) -> Vec<WallSegment> {
    let mut positions = Vec::new();
    let (hw, hd, step) = (config.half_width, config.half_depth, config.step);
    let mut curr_idx = 0;

    let mut add_wall = |pos: Vec3, rot_y: f32, is_corner: bool| {
        let wall_type = if config.door_indices.contains(&curr_idx) {
            WallType::Door
        } else if is_corner {
            WallType::Corner
        } else {
            WallType::Standard
        };

        positions.push(WallSegment {
            transform: Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(rot_y)),
            wall_type,
        });
        curr_idx += 1;
    };

    // West wall
    let mut z = -hd;
    while z <= hd {
        let is_southwest_corner = (z - -hd).abs() < ERROR_MARGIN;
        let is_northwest_corner = (z - hd).abs() < ERROR_MARGIN;
        let is_corner = is_southwest_corner || is_northwest_corner;

        let rot = if is_southwest_corner {
            0.0
        } else if is_northwest_corner {
            PI / 2.0
        } else {
            0.0
        };

        add_wall(Vec3::new(-hw, 0.0, z), rot, is_corner);
        z += step;
    }

    // North wall
    let mut x = -hw + step;
    while x <= hw {
        let is_northeast_corner = (x - hw).abs() < ERROR_MARGIN;

        let rot = if is_northeast_corner { PI } else { -PI / 2.0 };

        add_wall(Vec3::new(x, 0.0, hd), rot, is_northeast_corner);
        x += step;
    }

    // East wall
    z = hd - step;
    while z >= -hd {
        let is_southeast_corner = (z - -hd).abs() < ERROR_MARGIN;

        let rot = if is_southeast_corner { -PI / 2.0 } else { 0.0 };

        add_wall(Vec3::new(hw, 0.0, z), rot, is_southeast_corner);
        z -= step;
    }

    // South wall
    x = hw - step;
    while x > -hw {
        add_wall(Vec3::new(x, 0.0, -hd), PI / 2.0, false);
        x -= step;
    }

    positions
}

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

pub fn spawn_room(
    cmds: &mut Commands,
    asset_server: &Res<AssetServer>,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
    config: &RoomConfig,
    room_type: Rooms,
) {
    let layout = generate_rooms(config);

    cmds.spawn((
        Room { room_type },
        Transform::default(),
        Visibility::default(),
    ))
    .with_children(|parent| {
        for position in layout {
            let is_door = matches!(position.wall_type, WallType::Door);
            let asset_path = match position.wall_type {
                WallType::Standard => &config.wall_asset,
                WallType::Corner => &config.corner_asset,
                WallType::Door => &config.door_asset,
            };

            let mut entity = parent.spawn((
                WorldAssetRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path.clone())),
                ),
                position.transform.with_scale(SCALE),
            ));

            if is_door {
                entity.insert(Door);

                let mut graph = AnimationGraph::new();
                let mut indices = HashMap::new();

                for (i, name) in ["Door", "Door_Handles", "Door_inner_glass"]
                    .iter()
                    .enumerate()
                {
                    let clip = asset_server
                        .load(GltfAssetLabel::Animation(i).from_asset(asset_path.clone()));
                    let idx = graph.add_clip(clip, 1.0, graph.root);
                    indices.insert((*name).to_string(), idx);
                }

                let graph_handle = graphs.add(graph);
                entity.insert(DoorAnimation {
                    handle: graph_handle,
                    node_indices: indices,
                });
                entity.observe(door_animation_ready);
            }
        }

        for prop in &config.props {
            let mut entity = parent.spawn((
                WorldAssetRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset(prop.asset_path.clone())),
                ),
                prop.transform.with_scale(SCALE),
            ));

            if prop.texture_path.is_some() {
                entity.observe(apply_bookshelf_texture);
            }
        }
    });
}

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
        app.add_systems(Startup, setup_office)
            .add_systems(Update, open_door.run_if(input_just_pressed(KeyCode::KeyE)));
    }
}

fn setup_office(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let bookshelf_z = [-6.0, -3.0, 0.0, 3.0, 6.0];
    let mut props = vec![PropSpec {
        asset_path: "models/Office_Table.glb".into(),
        transform: Transform::from_translation(Vec3::new(5.0, 0.0, -0.5))
            .with_rotation(Quat::from_rotation_y(PI)),
        texture_path: None,
    }];

    for z in bookshelf_z {
        props.push(PropSpec {
            asset_path: "models/Office_Bookshelf.glb".into(),
            transform: Transform::from_translation(Vec3::new(15.0, 0.0, z))
                .with_rotation(Quat::from_rotation_y(PI / 2.0)),
            texture_path: Some("textures/Dark_Wood_texture.png".into()),
        });
    }
    let office_config = RoomConfig {
        name: "Office".into(),
        half_width: 16.0,
        half_depth: 8.0,
        step: 8.0,
        wall_asset: "models/Wall_office.glb".into(),
        corner_asset: "models/Wall_corner_1_office.glb".into(),
        door_asset: "models/Wall_office_door.glb".into(),
        door_indices: vec![1, 5],
        props,
    };
    spawn_room(
        &mut cmds,
        &asset_server,
        &mut graphs,
        &office_config,
        Rooms::Office(true),
    );
}

#[derive(Component)]
struct Bookshelf;

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

        if &material_name.0 == "Wooden" {
            cmds.entity(descendant)
                .insert(MeshMaterial3d(material.clone()));
        }
    }
}

fn door_animation_ready(
    scene_ready: On<WorldInstanceReady>,
    mut cmds: Commands,
    children: Query<&Children>,
    animations: Query<&DoorAnimation>,
    players: Query<&AnimationPlayer>,
) {
    let Ok(animation_data) = animations.get(scene_ready.entity) else {
        return;
    };

    for child in children.iter_descendants(scene_ready.entity) {
        if players.get(child).is_ok() {
            cmds.entity(child)
                .insert(AnimationGraphHandle(animation_data.handle.clone()))
                .insert(animation_data.clone());
        }
    }
}

fn open_door(
    mut door_query: Query<(&GlobalTransform, &mut AnimationPlayer, &DoorAnimation)>,
    cam_query: Single<(&Camera3d, &GlobalTransform)>,
) {
    let (_cam, cam_transform) = cam_query.into_inner();

    for (door_transform, mut player, animations) in &mut door_query {
        let cam_distance = cam_transform
            .translation()
            .distance(door_transform.translation());
        if cam_distance > 10.0 {
            continue;
        }

        let Some(&door_idx) = animations.node_indices.get("Door") else {
            continue;
        };

        play_animation(door_idx, &mut player, animations);
    }
}

fn play_animation(
    door_idx: AnimationNodeIndex,
    player: &mut AnimationPlayer,
    animations: &DoorAnimation,
) {
    if let Some(action) = player.animation(door_idx)
        && action.is_finished()
    {
        player.play(door_idx).replay();
        if let Some(&handle_idx) = animations.node_indices.get("Door_Handles") {
            player.play(handle_idx).replay();
        }
        return;
    }

    player.play(door_idx);
    if let Some(&handle_idx) = animations.node_indices.get("Door_Handles") {
        player.play(handle_idx);
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
    fn test_rooms_plugin_build() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, RoomsPlugin));
        app.update();
        assert!(app.is_plugin_added::<RoomsPlugin>());
    }
}
