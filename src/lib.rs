//! # Home Invasion
//!
//! A 3D horror game using the Bevy engine in collaboration with
//! [SQUA7426](https://github.com/SQUA7426).
//!
//! ## Examples
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_camera_controller::free_camera::FreeCameraPlugin;
//! use home_invasion::components::{cam::CamPlugin, house::HousePlugin};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins((
//!             (
//!                 (MinimalPlugins, FreeCameraPlugin),
//!                 (CamPlugin, HousePlugin)
//!             )
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
/// Component modules for the game.
pub mod components {
    pub mod cam;
    pub mod game_menu;
    pub mod house;
    pub mod player;
    pub mod rooms;
}

/// Utility modules such as the macros.
pub mod utils {
    pub mod macros;
}

use components::{cam::*, game_menu::*, house::*, player::*, rooms::*};
use utils::macros::*;
