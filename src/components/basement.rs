//! This module collects all functionalities of the Basement.

use super::rooms::{PropSpec, RoomConfig, Rooms, spawn_room};
use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

/// The [Plugin] of the Basement.
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
/// use home_invasion::components::basement::BasementPlugin;
///
/// App::new()
///     .add_plugins((
///         MinimalPlugins,
///         InputPlugin,
///         AssetPlugin::default(),
///         StatesPlugin,
///         BasementPlugin,
///     ))
///     .init_asset::<Image>()
///     .init_asset::<Mesh>()
///     .init_asset::<StandardMaterial>()
///     .init_asset::<AnimationGraph>()
///     .init_asset::<AnimationClip>()
///     .init_asset::<WorldAsset>()
///     .update();
/// ```
pub struct BasementPlugin;

impl Plugin for BasementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_basement);
    }
}

fn setup_basement(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let basement_config = RoomConfig {
        name: "Basement".into(),
        half_width: 64.0,
        half_depth: 16.0,
        step: 8.0,
        wall_asset: Some("models/Wall_office.glb".into()),
        corner_asset: Some("models/Wall_corner_1_office.glb".into()),
        door_asset: Some("models/Wall_office_door.glb".into()),
        door_indices: vec![1, 5],
        props: None,
        pos: Vec3::new(14.0, -14.0, -22.0),
    };
    spawn_room(
        &mut cmds,
        &asset_server,
        meshes,
        standard_materials,
        &mut graphs,
        &basement_config,
        Rooms::Basement(false),
    );
}
