//! This module collects all Rooms of the Second Floor.

use super::rooms::{PropSpec, RoomConfig, Rooms, spawn_room};
use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

/// The [Plugin] of the Second Floor.
///
/// # Examples
///
/// ```
/// use bevy::{
///   asset::AssetPlugin,
///   animation::AnimationClip,
///   input::InputPlugin,
///   prelude::*,
///   state::app::StatesPlugin,
///   world_serialization::WorldAsset,
/// };
/// use home_invasion::components::second_floor::SecondFloorPlugin;
///
/// App::new()
///     .add_plugins((
///         MinimalPlugins,
///         InputPlugin,
///         AssetPlugin::default(),
///         StatesPlugin,
///         SecondFloorPlugin,
///     ))
///     .init_asset::<Image>()
///     .init_asset::<Mesh>()
///     .init_asset::<StandardMaterial>()
///     .init_asset::<AnimationGraph>()
///     .init_asset::<AnimationClip>()
///     .init_asset::<WorldAsset>()
///     .update();
/// ```
pub struct SecondFloorPlugin;

impl Plugin for SecondFloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            OfficePlugin,
            Storage2Plugin,
            BedroomPlugin,
            KidsRoomPlugin,
            BathroomPlugin,
            Toilet2Plugin,
            Hallway2Plugin,
        ));
    }
}

struct OfficePlugin;

impl Plugin for OfficePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_office);
    }
}

fn setup_office(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let bookshelf_z = [-6.0, -3.0, 0.0, 3.0, 6.0];
    let mut props = vec![PropSpec {
        asset_path: "models/HomeOffice_Table.glb".into(),
        transform: Transform::from_translation(Vec3::new(5.0, 0.0, -0.5))
            .with_rotation(Quat::from_rotation_y(PI)),
        texture_path: None,
    }];

    for z in bookshelf_z {
        props.push(PropSpec {
            asset_path: "models/HomeOffice_Bookshelf.glb".into(),
            transform: Transform::from_translation(Vec3::new(15.0, 0.0, z))
                .with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
            texture_path: Some("textures/Dark_Wood_texture.png".into()),
        });
    }

    let office_config = RoomConfig {
        name: "Office".into(),
        half_width: 16.0,
        half_depth: 8.0,
        step: 8.0,
        wall_asset: Some("models/Wall_office.glb".into()),
        corner_asset: Some("models/Wall_corner_1_office.glb".into()),
        door_asset: Some("models/Wall_office_door.glb".into()),
        door_indices: vec![1, 5],
        props: Some(props),
        pos: Vec3::new(14.0, 14.0, -22.0),
    };
    spawn_room(
        &mut cmds,
        &asset_server,
        meshes,
        standard_materials,
        &mut graphs,
        &office_config,
        Rooms::Office(false),
    );
}

struct Storage2Plugin;

impl Plugin for Storage2Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_storage2);
    }
}

fn setup_storage2(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let storage2_config = RoomConfig {
        name: "Storage 2".into(),
        half_width: 16.0,
        half_depth: 8.0,
        step: 8.0,
        wall_asset: Some("models/Wall_office.glb".into()),
        corner_asset: Some("models/Wall_corner_1_office.glb".into()),
        door_asset: Some("models/Wall_office_door.glb".into()),
        door_indices: vec![10],
        props: None,
        pos: Vec3::new(14.0, 14.0, 34.0),
    };
    spawn_room(
        &mut cmds,
        &asset_server,
        meshes,
        standard_materials,
        &mut graphs,
        &storage2_config,
        Rooms::Storage2(false),
    );
}

struct BedroomPlugin;

impl Plugin for BedroomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bedroom);
    }
}

fn setup_bedroom(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let bedroom_config = RoomConfig {
        name: "Bedroom".into(),
        half_width: 16.0,
        half_depth: 8.0,
        step: 8.0,
        wall_asset: Some("models/Wall_office.glb".into()),
        corner_asset: Some("models/Wall_corner_1_office.glb".into()),
        door_asset: Some("models/Wall_office_door.glb".into()),
        door_indices: vec![10],
        props: None,
        pos: Vec3::new(-18.0, 14.0, 34.0),
    };

    spawn_room(
        &mut cmds,
        &asset_server,
        meshes,
        standard_materials,
        &mut graphs,
        &bedroom_config,
        Rooms::Bedroom(false),
    );
}

struct KidsRoomPlugin;

impl Plugin for KidsRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_kids_room);
    }
}

fn setup_kids_room(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let props = vec![PropSpec {
        asset_path: "models/table.glb".into(),
        transform: Transform::from_translation(Vec3::new(-2.0, 0.5, 0.0)),
        texture_path: None,
    }];
    let kids_room_config = RoomConfig {
        name: "Kid's Room".into(),
        half_width: 16.0,
        half_depth: 8.0,
        step: 8.0,
        wall_asset: Some("models/Wall_office.glb".into()),
        corner_asset: Some("models/Wall_corner_1_office.glb".into()),
        door_asset: Some("models/Wall_office_door.glb".into()),
        door_indices: vec![10],
        props: Some(props),
        pos: Vec3::new(-50.0, 14.0, 34.0),
    };

    spawn_room(
        &mut cmds,
        &asset_server,
        meshes,
        standard_materials,
        &mut graphs,
        &kids_room_config,
        Rooms::KidsRoom(false),
    );
}

struct BathroomPlugin;

impl Plugin for BathroomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bathroom);
    }
}

fn setup_bathroom(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let bathroom_config = RoomConfig {
        name: "Bathroom".into(),
        half_width: 8.0,
        half_depth: 36.0,
        step: 8.0,
        wall_asset: Some("models/Wall_office.glb".into()),
        corner_asset: Some("models/Wall_corner_1_office.glb".into()),
        door_asset: Some("models/Wall_office_door.glb".into()),
        door_indices: vec![5],
        props: None,
        pos: Vec3::new(38.0, 14.0, 6.0),
    };
    spawn_room(
        &mut cmds,
        &asset_server,
        meshes,
        standard_materials,
        &mut graphs,
        &bathroom_config,
        Rooms::Bathroom(false),
    );
}

struct Toilet2Plugin;

impl Plugin for Toilet2Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_toilet2);
    }
}

fn setup_toilet2(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let toilet2_config = RoomConfig {
        name: "Toilet 2".into(),
        half_width: 8.0,
        half_depth: 8.0,
        step: 8.0,
        wall_asset: Some("models/Wall_office.glb".into()),
        corner_asset: Some("models/Wall_corner_1_office.glb".into()),
        door_asset: Some("models/Wall_office_door.glb".into()),
        door_indices: vec![3],
        props: None,
        pos: Vec3::new(-40.0, 0.0, -34.0),
    };
    spawn_room(
        &mut cmds,
        &asset_server,
        meshes,
        standard_materials,
        &mut graphs,
        &toilet2_config,
        Rooms::Toilet2(false),
    );
}

struct Hallway2Plugin;

impl Plugin for Hallway2Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hallway2);
    }
}

fn setup_hallway2(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let hallway2_config = RoomConfig {
        name: "Hallway 2".into(),
        half_width: 70.0,
        half_depth: 32.0,
        step: 8.0,
        wall_asset: None,
        corner_asset: None,
        door_asset: None,
        door_indices: vec![5],
        props: None,
        pos: Vec3::new(-0.0, 0.0, 0.0),
    };
    spawn_room(
        &mut cmds,
        &asset_server,
        meshes,
        standard_materials,
        &mut graphs,
        &hallway2_config,
        Rooms::Hallway2(false),
    );
}
