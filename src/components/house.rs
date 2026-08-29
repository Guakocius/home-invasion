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
/// ```no_run
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
        app /*.add_systems(Startup, setup_floor)*/
            .add_plugins((AnimationsPlugin, RoomsPlugin));
    }
}

#[cfg(test)]
mod tests {
    use bevy::input::InputPlugin;

    use super::*;
}
