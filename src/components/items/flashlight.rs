//! The flashlight module.

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_audio::{AudioPlayer, PlaybackSettings};

use crate::{FlashLightAnimation, Player, SoundEffect, flashlight_animation_ready};

/// Item that represents the [`FlashLight`].
#[derive(Component, Debug)]
pub struct FlashLight;

/// [`FlashLight`] that is hold by [Player].
#[derive(Component, Debug)]
pub struct PlayerFlashLight;

/// Defines the State of the equipped [`FlashLight`]
#[derive(Debug, States, Clone, Default, PartialEq, Eq, Hash)]
pub enum FlashLightState {
    /// `FlashLightState On`.
    On,
    /// `FlashLightState Off`.
    #[default]
    Off,
}

/// Defines if [`FlashLight`] is `On`.
#[derive(Resource)]
pub struct FlashLightOn(pub bool);

/// [Plugin] containing [`FlashLight`] logics.
pub struct FlashLightPlugin;

impl Plugin for FlashLightPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<FlashLightState>()
            .insert_resource(FlashLightOn(false))
            .add_systems(Update, change_flashlight_state)
            .add_systems(OnEnter(FlashLightState::On), spawn_flashlight)
            .add_systems(Update, flashlight_light)
            .add_systems(Update, move_flashlight)
            .add_systems(Update, despawn_flashlight);
    }
}

fn spawn_flashlight(
    mut cmds: Commands,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
    player: Single<Entity, With<Player>>,
) {
    let player_entity = player.into_inner();

    let asset_path: String = "models/flashlight.glb".into();
    let mut graph = AnimationGraph::new();
    let mut indices: HashMap<String, AnimationNodeIndex> = HashMap::new();

    for (i, name) in ["ON_OFF", "FlashlightAction"].iter().enumerate() {
        let clip = asset_server.load(GltfAssetLabel::Animation(i).from_asset(asset_path.clone()));

        let idx = graph.add_clip(clip, 1.0, graph.root);
        indices.insert((*name).to_string(), idx);
    }

    let graph_handle = graphs.add(graph);

    cmds.entity(player_entity)
        .with_children(|parent| {
            parent.spawn((
                PlayerFlashLight,
                WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path))),
                Transform::from_translation(Vec3::new(-0.2, -0.3, -0.3))
                    .with_scale(Vec3::new(0.1, 0.1, 0.1))
                    .looking_at(Vec3::new(-10.0, 10.0, 0.0), Vec3::Y),
                FlashLight,
                FlashLightAnimation {
                    handle: graph_handle,
                    node_indices: indices,
                },
            ));
        })
        .observe(flashlight_animation_ready);
}

fn change_flashlight_state(
    mut cmds: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    sound_effect: Res<SoundEffect>,
    mut flashlight_state: ResMut<NextState<FlashLightState>>,
    flashlight_on: Res<FlashLightOn>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        cmds.spawn((
            AudioPlayer::new(sound_effect.clone()),
            PlaybackSettings::DESPAWN,
        ));
        flashlight_state.set(if flashlight_on.0 {
            FlashLightState::Off
        } else {
            FlashLightState::On
        });
        cmds.insert_resource(FlashLightOn(!flashlight_on.0));
    }
}

fn flashlight_light(
    mut cmds: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<&Transform, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        let transform = player.into_inner();
        cmds.spawn((
            DespawnOnExit(FlashLightState::On),
            FlashLight,
            SpotLight {
                color: Color::srgb(1.0, 1.0, 1.0),
                range: 50.0,
                radius: 100.0,
                inner_angle: 0.15,
                outer_angle: 0.75,
                shadow_maps_enabled: true,
                ..default()
            },
            Transform::from_translation(transform.translation).with_rotation(transform.rotation),
        ));
    }
}

type PlayerFlashLightTransform<'a> = (&'a mut Transform, Has<PlayerFlashLight>);
type WithFlashLight = (With<FlashLight>, Without<Player>);

fn move_flashlight(
    mut flashlight_query: Query<PlayerFlashLightTransform, WithFlashLight>,
    player: Single<&Transform, (With<Player>, Without<FlashLight>)>,
    flashlight_on: Res<FlashLightOn>,
) {
    if !flashlight_on.0 {
        return;
    }
    let player_transform = player.into_inner();
    for (mut flashlight_transform, has_flashlight) in &mut flashlight_query {
        if !has_flashlight {
            flashlight_transform.translation = player_transform.translation;
            flashlight_transform.rotation = player_transform.rotation;
        }
    }
}

fn despawn_flashlight(
    mut cmds: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    flashlight_query: Single<(Entity, &FlashLight), With<PlayerFlashLight>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        cmds.entity(flashlight_query.into_inner().0).despawn();
    }
}
