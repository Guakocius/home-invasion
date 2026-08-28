//! This module defines the room layout and the logic of each of the house's rooms.

use bevy::{
    color::palettes::css::WHITE,
    ecs::event::Trigger,
    gltf::{GltfExtras, GltfMaterialExtras, GltfMaterialName},
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    input::InputPlugin,
    math::Affine2,
    platform::collections::HashMap,
    prelude::{Component, States, *},
    state::app::StatesPlugin,
    world_serialization::WorldInstanceReady,
};
use bevy_rapier3d::geometry::Collider;
use std::{
    f32::consts::{FRAC_PI_2, PI},
    fmt,
};

use super::{
    animations::{DoorAnimation, door_animation_ready},
    basement::BasementPlugin,
    first_floor::FirstFloorPlugin,
    second_floor::SecondFloorPlugin,
};

const ERROR_MARGIN: f32 = 0.1;
const SCALE: Vec3 = Vec3::new(4.0, 4.0, 4.0);

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
    /// The house's dining room.
    DiningRoom(bool),
    /// The house's first floor hallway.
    Hallway1(bool),
    /// The house's floor hallway.
    Hallway2(bool),
    /// The house's home office on the first floor.
    HomeOffice(bool),
    /// The house's kid's room.
    KidsRoom(bool),
    /// The house's kitchen.
    Kitchen(bool),
    /// The house's living room.
    LivingRoom(bool),
    /// The house's office on the second floor.
    Office(bool),
    /// The house's shower.
    Shower(bool),
    /// The house's first floor storage.
    Storage1(bool),
    /// The house's second floor storage.
    Storage2(bool),
    /// The house's first floor toilet.
    Toilet1(bool),
    /// The house's second floor toilet.
    Toilet2(bool),
}

#[derive(Component, Debug, Clone, Copy, Default)]
struct Wall;

#[derive(Component, Debug, Clone, Copy, Default)]
struct Door;

/// This structure defines each `Room` and its contents.
///
/// # Examples
///
/// ```
/// use home_invasion::components::rooms::{Room, Rooms};
///
/// let room = Room { room_type: Rooms::Office(true) };
/// assert_eq!(room.room_type, Rooms::Office(true));
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
            Rooms::DiningRoom(_) => "Dining Room",
            Rooms::Hallway1(_) => "Hallway 1",
            Rooms::Hallway2(_) => "Hallway 2",
            Rooms::HomeOffice(_) => "Home Office",
            Rooms::KidsRoom(_) => "Kid's Room",
            Rooms::Kitchen(_) => "Kitchen",
            Rooms::LivingRoom(_) => "Living Room",
            Rooms::Office(_) => "Office",
            Rooms::Shower(_) => "Shower",
            Rooms::Storage1(_) => "Storage 1",
            Rooms::Storage2(_) => "Storage 2",
            Rooms::Toilet1(_) => "Toilet 1",
            Rooms::Toilet2(_) => "Toilet 2",
        };
        write!(f, "{name}")
    }
}

/// Plugin for all systems associated with the [`Rooms`].
///
/// # Examples
///
/// ```
/// use bevy::{
///   asset::AssetPlugin,
///   input::InputPlugin,
///   prelude::*,
///   state::app::StatesPlugin,
/// };
/// use home_invasion::components::rooms::RoomsPlugin;
///
/// App::new()
///   .add_plugins((
///     MinimalPlugins,
///     InputPlugin,
///     AssetPlugin::default(),
///     StatesPlugin,
///     RoomsPlugin,
///   ))
///   .init_asset::<Image>()
///   .init_asset::<Mesh>()
///   .init_asset::<StandardMaterial>()
///   .init_asset::<AnimationGraph>()
///   .init_asset::<AnimationClip>()
///   .init_asset::<WorldAsset>()
///   .update();
/// ```
pub struct RoomsPlugin;

impl Plugin for RoomsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimationGraph>()
            .init_asset::<AnimationClip>()
            .init_asset::<WorldAsset>()
            .insert_state(Rooms::Office(true))
            .add_plugins((BasementPlugin, FirstFloorPlugin, SecondFloorPlugin));
    }
}

/// The type of `Wall` applied.
///
/// # Examples
///
/// ```
/// use home_invasion::components::rooms::WallType;
///
/// let wall_type = WallType::Standard;
///
/// match wall_type {
///   WallType::Standard => println!("Standard"),
///   WallType::Corner => println!("Corner"),
///   WallType::Door => println!("Door"),
/// }
/// ````
pub enum WallType {
    /// The standard `Wall` type.
    Standard,
    /// The `Wall` type of corners.
    Corner,
    /// The `Wall` type of `Doors`.
    Door,
}

/// A segment of a `Wall` (north, east, south, west).
///
/// # Examples
///
/// ```
/// use std::f32::consts::FRAC_PI_2;
/// use bevy::prelude::*;
/// use home_invasion::components::rooms::{WallSegment, WallType};
///
/// let wall_type = WallType::Standard;
/// let pos = Vec3::ZERO;
///
/// let wall_segment = WallSegment {
///   transform: Transform::from_translation(pos)
///     .with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
///   wall_type,
/// };
/// ```
pub struct WallSegment {
    /// The positional transformation of the `Wall`.
    pub transform: Transform,
    /// The [type of Wall](WallType).
    pub wall_type: WallType,
}

/// The specifications of the props such as the bookshelfs and tables.
///
/// # Examples
///
/// ```
/// use std::f32::consts::PI;
/// use bevy::prelude::*;
/// use home_invasion::components::rooms::PropSpec;
///
/// let mut props = vec![PropSpec {
///   asset_path: "models/Office_Table.glb".into(),
///   transform: Transform::from_translation(Vec3::new(5.0, 0.0, -0.5))
///     .with_rotation(Quat::from_rotation_y(PI)),
///   texture_path: None,
/// }];
/// ```
#[derive(Clone)]
pub struct PropSpec {
    /// The path of the asset.
    pub asset_path: String,
    /// The prop's positional transformation.
    pub transform: Transform,
    /// The optional path of the prop's texture.
    pub texture_path: Option<String>,
}

/// The configuration of a `Room`.
///
/// # Examples
///
/// ```
/// use std::f32::consts::PI;
/// use bevy::prelude::*;
/// use home_invasion::components::rooms::{PropSpec, RoomConfig};
///
/// let mut props = vec![PropSpec {
///   asset_path: "models/Office_Table.glb".into(),
///   transform: Transform::from_translation(Vec3::new(5.0, 0.0, -0.5))
///     .with_rotation(Quat::from_rotation_y(PI)),
///   texture_path: None,
/// }];
///
/// let config = RoomConfig {
///   name: "Office".into(),
///   half_width: 16.0,
///   half_depth: 8.0,
///   step: 8.0,
///   floor_asset: Some("textures/wooden_plank_floor.png".into()),
///   wall_asset: Some("models/Wall_office.glb".into()),
///   corner_asset: Some("models/Wall_corner_1_office.glb".into()),
///   door_asset: Some("models/Wall_office_door.glb".into()),
///   door_indices: vec![1, 5],
///   props: Some(props),
///   pos: Vec3::ZERO,
/// };
/// ```
#[derive(Clone)]
pub struct RoomConfig {
    /// The name of the `Room`.
    pub name: String,
    /// The half width of the `Room`.
    pub half_width: f32,
    /// The half depth of the `Room`.
    pub half_depth: f32,
    /// The physical width of a single [`WallSegment`].
    pub step: f32,
    /// The asset path of the `Floor`.
    pub floor_asset: Option<String>,
    /// The asset path of the `Wall`.
    pub wall_asset: Option<String>,
    /// The asset path of the corners.
    pub corner_asset: Option<String>,
    /// The asset path of the `Doors`.
    pub door_asset: Option<String>,
    /// A [Vec] specifying which `Wall` positions around the `Room's` perimeter should be
    /// spawned as doors.
    pub door_indices: Vec<usize>,
    /// The `Room's` props.
    pub props: Option<Vec<PropSpec>>,
    /// The `Room's` position in the world.
    pub pos: Vec3,
}

/// Generate rooms based on the given parameters.
///
/// # Examples
///
/// ```
/// use std::f32::consts::PI;
/// use bevy::prelude::*;
/// use home_invasion::components::rooms::{PropSpec, RoomConfig, generate_rooms};
///
/// let mut props = vec![PropSpec {
///   asset_path: "models/Office_Table.glb".into(),
///   transform: Transform::from_translation(Vec3::new(5.0, 0.0, -0.5))
///     .with_rotation(Quat::from_rotation_y(PI)),
///   texture_path: None,
/// }];
///
/// let config = RoomConfig {
///   name: "Office".into(),
///   half_width: 16.0,
///   half_depth: 8.0,
///   step: 8.0,
///   floor_asset: None,
///   wall_asset: Some("models/Wall_office.glb".into()),
///   corner_asset: Some("models/Wall_corner_1_office.glb".into()),
///   door_asset: Some("models/Wall_office_door.glb".into()),
///   door_indices: vec![1, 5],
///   props: Some(props),
///   pos: Vec3::ZERO,
/// };
/// let room = generate_rooms(&config);
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
            FRAC_PI_2
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

        let rot = if is_northeast_corner { PI } else { -FRAC_PI_2 };

        add_wall(Vec3::new(x, 0.0, hd), rot, is_northeast_corner);
        x += step;
    }

    // East wall
    z = hd - step;
    while z >= -hd {
        let is_southeast_corner = (z - -hd).abs() < ERROR_MARGIN;

        let rot = if is_southeast_corner { -FRAC_PI_2 } else { 0.0 };

        add_wall(Vec3::new(hw, 0.0, z), rot, is_southeast_corner);
        z -= step;
    }

    // South wall
    x = hw - step;
    while x > -hw {
        add_wall(Vec3::new(x, 0.0, -hd), FRAC_PI_2, false);
        x -= step;
    }

    positions
}

/// A function spawning a `Room` with all its props, `Walls`, `Doors` and all the `Doors'` animations.
///
/// # Examples
///
/// ```
/// use std::f32::consts::PI;
/// use bevy::{input::InputPlugin, prelude::*, state::app::StatesPlugin};
/// use home_invasion::components::rooms::{PropSpec, RoomConfig, Rooms, spawn_room};
///
/// fn setup(
///   mut cmds: Commands,
///   asset_server: Res<AssetServer>,
///   meshes: ResMut<Assets<Mesh>>,
///   standard_materials: ResMut<Assets<StandardMaterial>>,
///   mut graphs: ResMut<Assets<AnimationGraph>>,
/// ) {
///   let mut props = vec![PropSpec {
///     asset_path: "models/Office_Table.glb".into(),
///     transform: Transform::from_translation(Vec3::new(5.0, 0.0, -0.5))
///       .with_rotation(Quat::from_rotation_y(PI)),
///     texture_path: None,
///   }];
///
///   let office_config = RoomConfig {
///     name: "HomeOffice".into(),
///     half_width: 16.0,
///     half_depth: 8.0,
///     step: 8.0,
///     floor_asset: None,
///     wall_asset: Some("models/Wall_office.glb".into()),
///     corner_asset: Some("models/Wall_corner_1_office.glb".into()),
///     door_asset: Some("models/Wall_office_door.glb".into()),
///     door_indices: vec![1, 5],
///     props: Some(props),
///     pos: Vec3::ZERO,
///   };
///   spawn_room(
///     &mut cmds,
///     &asset_server,
///     meshes,
///     standard_materials,
///     &mut graphs,
///     &office_config,
///     Rooms::HomeOffice(true),
///   );
/// }
/// App::new()
///   .add_plugins((
///     MinimalPlugins,
///     InputPlugin,
///     AssetPlugin::default(),
///     StatesPlugin,
///   ))
///   .init_asset::<Image>()
///   .init_asset::<Mesh>()
///   .init_asset::<StandardMaterial>()
///   .init_asset::<AnimationGraph>()
///   .init_asset::<AnimationClip>()
///   .init_asset::<WorldAsset>()
///   .add_systems(Startup, setup)
///   .update();
/// ```
pub fn spawn_room(
    cmds: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
    config: &RoomConfig,
    room_type: Rooms,
) {
    let layout = generate_rooms(config);

    let floor_texture: String = match config.floor_asset.clone() {
        Some(f) => f,
        None    => "textures/wooden_plank_floor.png".into()
    };

    cmds.spawn((
        Room {
            room_type: room_type.clone(),
        },
        Transform::from_translation(config.pos),
        Visibility::default(),
        Wall,
    ))
    .with_children(|parent| {
        if config.wall_asset.is_some() {
            for position in layout {
                let is_door = matches!(position.wall_type, WallType::Door);
                let asset_path = match position.wall_type {
                    WallType::Standard => &config.wall_asset,
                    WallType::Corner => &config.corner_asset,
                    WallType::Door => &config.door_asset,
                };

                if let Some(asset_path) = asset_path {
                    let mut entity = parent.spawn((
                        WorldAssetRoot(
                            asset_server
                                .load(GltfAssetLabel::Scene(0).from_asset(asset_path.clone())),
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
            }
        }
        if let Some(props) = &config.props {
            for prop in props {
                let mut entity = parent.spawn((
                    WorldAssetRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset(prop.asset_path.clone())),
                    ),
                    prop.transform.with_scale(SCALE),
                ));

                if prop.texture_path.is_some() {
                    entity.observe(apply_bookshelf_texture);
                }
            }
        }
    });

    let y: f32 = match room_type {
        Rooms::Hallway1(_) | Rooms::Hallway2(_) => 0.0,
        _ => 0.005,
    };
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            Vec3::Y,
            Vec2::new(config.half_width, config.half_depth),
        ))),
        MeshMaterial3d(
            standard_materials.add(StandardMaterial {
                base_color: Color::from(bevy::color::palettes::css::WHITE),
                base_color_texture: Some(
                    asset_server
                        .load_builder()
                        .with_settings(configure_floor_texture_settings)
                        .load(floor_texture),
                ),
                uv_transform: Affine2::from_scale(vec2(10.0, 10.0)),
                perceptual_roughness: 0.8,
                ..default()
            }),
        ),
        Transform::from_translation(config.pos + y),
        Visibility::Visible,
    ))
    .with_children(|parent| {
        parent
            .spawn(Collider::cuboid(config.half_width, 0.1, config.half_depth))
            .insert(Transform::from_xyz(0.0, 0.0, 0.0));
    });
}

fn configure_floor_texture_settings(s: &mut ImageLoaderSettings) {
    *s = ImageLoaderSettings {
        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            ..default()
        }),
        ..default()
    };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rooms_fmt() {
        assert_eq!(format!("{}", Rooms::Basement(true)), "Basement");
        assert_eq!(format!("{}", Rooms::Bathroom(true)), "Bathroom");
        assert_eq!(format!("{}", Rooms::Bedroom(true)), "Bedroom");
        assert_eq!(format!("{}", Rooms::Hallway1(true)), "Hallway 1");
        assert_eq!(format!("{}", Rooms::Hallway2(true)), "Hallway 2");

        assert_eq!(format!("{}", Rooms::HomeOffice(true)), "Home Office");
        assert_eq!(format!("{}", Rooms::KidsRoom(true)), "Kid's Room");
        assert_eq!(format!("{}", Rooms::Kitchen(true)), "Kitchen");
        assert_eq!(format!("{}", Rooms::LivingRoom(true)), "Living Room");
        assert_eq!(format!("{}", Rooms::Office(true)), "Office");
        assert_eq!(format!("{}", Rooms::Shower(true)), "Shower");
        assert_eq!(format!("{}", Rooms::Storage1(true)), "Storage 1");
        assert_eq!(format!("{}", Rooms::Storage2(true)), "Storage 2");
        assert_eq!(format!("{}", Rooms::Toilet1(true)), "Toilet 1");
        assert_eq!(format!("{}", Rooms::Toilet2(true)), "Toilet 2");
    }
}
