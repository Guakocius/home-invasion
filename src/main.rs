use bevy::prelude::*;
use bevy_camera_controller::free_camera::FreeCameraPlugin;

use home_invasion::components::{cam::CamPlugin, house::HousePlugin, rooms::office::OfficePlugin};

pub fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        (
            DefaultPlugins.set(AssetPlugin {
                mode: AssetMode::Unprocessed,
                ..default()
            }),
            FreeCameraPlugin,
        ),
        (CamPlugin, HousePlugin, OfficePlugin),
    ));
    app
}

fn main() {
    build_app().run();
}
