//! This module defines the structure and the logic of the house.

use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    math::Affine2,
    prelude::*,
    state::app::StatesPlugin,
};
use bevy_rapier3d::prelude::Collider;

use crate::components::{animations::AnimationsPlugin, rooms::RoomsPlugin};

/// Plugin for the house's systems.
///
/// # Examples
///
/// ```
/// use bevy::{
///   asset::AssetPlugin,
///   image::Image,
///   input::InputPlugin,
///   prelude::*,
///   state::app::StatesPlugin
/// };
/// use home_invasion::components::house::HousePlugin;
///
/// App::new()
///     .add_plugins((
///       MinimalPlugins,
///       InputPlugin,
///       AssetPlugin::default(),
///       StatesPlugin,
///       HousePlugin
///     ))
///     .init_asset::<WorldAsset>()
///     .init_asset::<Mesh>()
///     .init_asset::<StandardMaterial>()
///     .init_asset::<Image>()
///     .update()
/// ```
pub struct HousePlugin;

impl Plugin for HousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_floor)
            .add_plugins((AnimationsPlugin, RoomsPlugin));
    }
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

fn setup_floor(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::new(0.0, 10.0, 0.0), Vec2::splat(2.0)))),
        MeshMaterial3d(
            standard_materials.add(StandardMaterial {
                base_color: Color::from(bevy::color::palettes::css::WHITE),
                base_color_texture: Some(
                    asset_server
                        .load_builder()
                        .with_settings(configure_floor_texture_settings)
                        .load("textures/wooden_plank_floor.png"),
                ),
                uv_transform: Affine2::from_scale(vec2(10.0, 10.0)),
                perceptual_roughness: 0.8,
                ..default()
            }),
        ),
        Transform::from_scale(Vec3::splat(15.0)),
        Visibility::Visible,
    ))
    .with_children(|children| {
        children
            .spawn(Collider::cuboid(1.0, 0.0, 1.0))
            .insert(Transform::from_xyz(0.0, 0.0, 0.0));
    });
}

#[cfg(test)]
mod tests {
    use bevy::input::InputPlugin;

    use super::*;

    #[test]
    fn test_house_plugin_build() {
        let mut app = App::new();

        app.add_plugins((
            MinimalPlugins,
            InputPlugin,
            AssetPlugin::default(),
            StatesPlugin,
            HousePlugin,
        ))
        .init_asset::<WorldAsset>()
        .init_asset::<Mesh>()
        .init_asset::<StandardMaterial>()
        .init_asset::<Image>()
        .update();

        assert!(app.is_plugin_added::<AssetPlugin>());
        assert!(app.is_plugin_added::<HousePlugin>());
    }

    #[test]
    fn test_configure_floor_texture_settings() {
        let mut settings = ImageLoaderSettings::default();

        configure_floor_texture_settings(&mut settings);

        assert!(matches!(
            settings.sampler,
            ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..
            })
        ));
    }
}
