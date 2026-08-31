//! Module for all animations.
use bevy::{
    animation::{AnimationPlayer, graph::AnimationGraph},
    input::common_conditions::input_just_pressed,
    platform::collections::HashMap,
    prelude::*,
    world_serialization::WorldInstanceReady,
};

use crate::components::items::flashlight::{FlashLightOn, PlayerFlashLight};

use super::player::Player;

/// [Plugin] containing all animations.
///
/// # Examples
///
/// ```no_run
/// use bevy::{
///   asset::AssetPlugin,
///   input::InputPlugin,
///   prelude::*,
///   state::app::StatesPlugin,
///   world_serialization::WorldAsset,
/// };
/// use home_invasion::components::animations::AnimationsPlugin;
///
/// App::new()
///   .add_plugins((
///     MinimalPlugins,
///     InputPlugin,
///     AssetPlugin::default(),
///     StatesPlugin,
///     AnimationsPlugin,
///   ))
///   .init_asset::<AnimationGraph>()
///   .init_asset::<AnimationClip>()
///   .init_asset::<WorldAsset>()
///   .update();
/// ```
pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, open_door.run_if(input_just_pressed(KeyCode::KeyE)))
            .add_systems(
                Update,
                flashlight_player.run_if(input_just_pressed(KeyCode::KeyF)),
            );
    }
}

/// Structure containing the door animation handler and the mapped door nodes for the [Handle].
#[derive(Component, Clone)]
pub struct DoorAnimation {
    /// The `Doors'` animation handler.
    pub handle: Handle<AnimationGraph>,
    /// Maps all door sub-components into their specific node ID inside the [`AnimationGraph`].
    pub node_indices: HashMap<String, AnimationNodeIndex>,
}

/// Observer function signalling whether the door's GLTF scene finished spawning.
///
/// # Examples
///
/// ```no_run
/// use bevy::{
///   asset::AssetPlugin,
///   input::InputPlugin,
///   prelude::*,
///   state::app::StatesPlugin,
///   world_serialization::WorldAsset,
/// };
/// use home_invasion::components::animations::door_animation_ready;
///
/// const SCALE: Vec3 = Vec3::new(4.0, 4.0, 4.0);
///
/// fn observe_entity(
///   mut cmds: Commands,
///   asset_server: Res<AssetServer>,
/// ) {
///   let position = Vec3::ZERO;
///   let mut entity = cmds.spawn((
///     WorldAssetRoot(
///       asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Wall_office.glb")),
///     ),
///     Transform::from_translation(position).with_scale(SCALE),
///   ));
///   entity.observe(door_animation_ready);
/// }
///
/// App::new()
///   .add_plugins((
///     MinimalPlugins,
///     InputPlugin,
///     AssetPlugin::default(),
///     StatesPlugin,
///   ))
///   .init_asset::<AnimationGraph>()
///   .init_asset::<AnimationClip>()
///   .init_asset::<WorldAsset>()
///   .add_systems(Update, observe_entity)
///   .update();
/// ```
pub fn door_animation_ready(
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
    player_query: Single<&GlobalTransform, With<Player>>,
) {
    let player_transform = player_query.into_inner();

    for (door_transform, mut player, animations) in &mut door_query {
        let cam_distance = player_transform
            .translation()
            .distance(door_transform.translation());
        if cam_distance > 6.0 {
            continue;
        }

        let Some(&door_idx) = animations.node_indices.get("Door") else {
            continue;
        };

        play_door_animation(door_idx, &mut player, animations);
    }
}

fn play_door_animation(
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

/// Structure containing the flashlight animation handler and the mapped door nodes for the [Handle].
#[derive(Component, Clone)]
pub struct FlashLightAnimation {
    pub handle: Handle<AnimationGraph>,
    pub node_indices: HashMap<String, AnimationNodeIndex>,
}

/// Observer function signalling whether the flashlight's GLTF scene finished spawning.
pub fn flashlight_animation_ready(
    scene_ready: On<WorldInstanceReady>,
    mut cmds: Commands,
    children: Query<&Children>,
    animations: Query<&FlashLightAnimation>,
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

/// Searches for needed FlashLightAnimation.
pub fn flashlight_player(
    flashlight_query: Single<(&mut AnimationPlayer, &FlashLightAnimation), With<PlayerFlashLight>>,
    flashlight_on: Res<FlashLightOn>,
) {
    info!("Flashlight PLAYER...");
    let (mut player, animations) = flashlight_query.into_inner();
    let Some(&flashlight_idx1) = animations.node_indices.get("ON_OFF") else {
        return;
    };

    let Some(&flashlight_idx2) = animations.node_indices.get("FlashlightAction") else {
        return;
    };

    play_flashlight_animation(flashlight_idx1, &mut player, flashlight_on.0);
    play_flashlight_animation(flashlight_idx2, &mut player, flashlight_on.0);
}

fn play_flashlight_animation(flashlight_idx: AnimationNodeIndex, player: &mut AnimationPlayer, on: bool) {
    if let Some(action) = player.animation(flashlight_idx)
        && (action.is_finished() || action.is_paused())
    {
        if on {
            info!("Flashlight ON");
            player.play(flashlight_idx).replay();
            player.adjust_speeds(-1.0);
        } else {
            info!("Flashlight OFF");
            player.play(flashlight_idx);
            player.adjust_speeds(1.0);
        }
        return;
    }

    player.play(flashlight_idx);
    player.adjust_speeds(-1.0);
}
