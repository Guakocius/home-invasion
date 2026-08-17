//! This module collects all Rooms of the First Floor.
use super::rooms::{PropSpec, RoomConfig, Rooms, spawn_room};
use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

/// The [Plugin] of the First Floor.
pub struct FirstFloorPlugin;

impl Plugin for FirstFloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HomeOfficePlugin,
            LivingRoomPlugin,
            Storage1Plugin,
            DiningRoomPlugin,
            KitchenPlugin,
            ToiletPlugin,
            ShowerPlugin,
            HallwayPlugin,
        ));
    }
}

struct HomeOfficePlugin;

impl Plugin for HomeOfficePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_home_office);
    }
}

fn setup_home_office(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
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
        name: "Home Office".into(),
        half_width: 16.0,
        half_depth: 8.0,
        step: 8.0,
        wall_asset: "models/Wall_office.glb".into(),
        corner_asset: "models/Wall_corner_1_office.glb".into(),
        door_asset: "models/Wall_office_door.glb".into(),
        door_indices: vec![1, 5],
        props,
        pos: Vec3::new(14.0, 0.0, -22.0),
    };
    spawn_room(
        &mut cmds,
        &asset_server,
        &mut graphs,
        &office_config,
        Rooms::HomeOffice(true),
    );
}

struct LivingRoomPlugin;

impl Plugin for LivingRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_living_room);
    }
}

fn setup_living_room(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
}

struct Storage1Plugin;

impl Plugin for Storage1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_storage1);
    }
}

fn setup_storage1(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
}

struct DiningRoomPlugin;

impl Plugin for DiningRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_dining_room);
    }
}

fn setup_dining_room(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
}

struct KitchenPlugin;

impl Plugin for KitchenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_kitchen);
    }
}

fn setup_kitchen(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
}

struct ToiletPlugin;

impl Plugin for ToiletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_toilet);
    }
}

fn setup_toilet(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
}

struct ShowerPlugin;

impl Plugin for ShowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_shower);
    }
}

fn setup_shower(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
}

struct HallwayPlugin;

impl Plugin for HallwayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hallway);
    }
}

fn setup_hallway(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
}
