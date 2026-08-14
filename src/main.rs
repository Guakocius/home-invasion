use bevy::prelude::*;
use bevy_camera_controller::free_camera::FreeCameraPlugin;

use home_invasion::{
    cfg::window_plugin::build_platform_window_plugin,
    components::{cam::CamPlugin, house::HousePlugin, player::PlayerPlugin},
};

pub fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                mode: AssetMode::Unprocessed,
                ..default()
            })
            .set(build_platform_window_plugin()),
        FreeCameraPlugin,
        CamPlugin,
        HousePlugin,
        PlayerPlugin,
    ));
    app
}

fn main() {
    build_app().run();
}
