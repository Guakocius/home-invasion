//! Module collecting the [Plugins](Plugin) and their logic and functionality.

pub mod flashlight;

use bevy::prelude::*;
use flashlight::FlashLightPlugin;

/// [Plugin] containing all items.
pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FlashLightPlugin);
    }
}
