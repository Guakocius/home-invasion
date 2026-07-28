use bevy::prelude::*;
use bevy_camera_controller::free_camera::FreeCameraPlugin;

use home_invasion::components::{cam::CamPlugin, house::HousePlugin};

pub fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins(((DefaultPlugins, FreeCameraPlugin), (CamPlugin, HousePlugin)));
    app
}

fn main() {
    build_app().run();
}
