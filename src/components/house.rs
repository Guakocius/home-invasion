use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    math::Affine2,
    prelude::*,
};
use bevy_rapier3d::prelude::Collider;

///
pub struct HousePlugin;

impl Plugin for HousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_floor);
    }
}

fn setup_floor(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::new(0., 10., 0.), Vec2::splat(2.)))),
        MeshMaterial3d(
            standard_materials.add(StandardMaterial {
                base_color: Color::from(bevy::color::palettes::css::WHITE),
                base_color_texture: Some(
                    asset_server
                        .load_builder()
                        .with_settings(|s: &mut _| {
                            *s = ImageLoaderSettings {
                                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                                    address_mode_u: ImageAddressMode::Repeat,
                                    address_mode_v: ImageAddressMode::Repeat,
                                    ..default()
                                }),
                                ..default()
                            }
                        })
                        .load("textures/wooden_plank_floor.png"),
                ),
                uv_transform: Affine2::from_scale(vec2(10., 10.)),
                perceptual_roughness: 0.8,
                ..default()
            }),
        ),
        Transform::from_scale(Vec3::splat(10.)),
        Visibility::Visible,
    ))
    .with_children(|children| {
        children
            .spawn(Collider::cuboid(1., 0., 1.))
            .insert(Transform::from_xyz(0., 0., 0.));
    });
}

#[cfg(test)]
mod tests {}
