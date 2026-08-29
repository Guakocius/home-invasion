use bevy::prelude::*;

use home_invasion::{
    cfg::window_plugin::build_platform_window_plugin,
    components::{
        cam::CamPlugin, game_menu::GameMenuPlugin, house::HousePlugin, player::PlayerPlugin,
    },
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
        GameMenuPlugin,
        CamPlugin,
        HousePlugin,
        PlayerPlugin,
    ));
    app
}

fn main() {
    build_app().run();
}
