use bevy::prelude::*;
use bevy_camera_controller::free_camera::FreeCameraPlugin;

use home_invasion::components::{cam::CamPlugin, house::HousePlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FreeCameraPlugin))
        .add_plugins((CamPlugin, HousePlugin))
        .run();
}
