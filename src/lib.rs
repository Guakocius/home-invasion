//! # Home Invasion
//!
//! A 3D horror game using the Bevy engine in collaboration with
//! [SQUA7426](https://github.com/SQUA7426).
//!
//! ## Examples
//!
//! ```
//! use bevy::{
//!   asset::AssetPlugin,
//!   input::InputPlugin,
//!   prelude::*,
//!   state::app::StatesPlugin,
//! };
//! use home_invasion::components::{cam::CamPlugin, house::HousePlugin};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins((
//!                 MinimalPlugins,
//!                 InputPlugin,
//!                 AssetPlugin::default(),
//!                 StatesPlugin,
//!                 CamPlugin,
//!                 HousePlugin,
//!         ));
//! }
//! ```
//!
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::empty_docs)]
#![allow(unused_imports)]
/// Modules for environment-specific configurations.
pub mod cfg {
    pub mod window_plugin;
}

/// Component modules for the game.
pub mod components {
    pub mod animations;
    pub mod basement;
    pub mod cam;
    pub mod first_floor;
    pub mod game_menu;
    pub mod house;
    pub mod items;
    pub mod player;
    pub mod rooms;
    pub mod second_floor;
    pub mod sound;
}

/// Utility modules such as the macros.
pub mod utils {
    pub mod macros;
}

use cfg::window_plugin::*;
use components::{
    animations::{
        DoorAnimation, FlashLightAnimation, door_animation_ready, flashlight_animation_ready,
    },
    basement::BasementPlugin,
    cam::*,
    first_floor::FirstFloorPlugin,
    game_menu::*,
    house::*,
    items::{
        flashlight::{FlashLightOn, FlashLightPlugin, PlayerFlashLight},
        *,
    },
    player::*,
    rooms::*,
    second_floor::SecondFloorPlugin,
    sound::{SoundEffect, SoundPlugin},
};
use utils::macros::*;
