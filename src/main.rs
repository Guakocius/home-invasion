use bevy::prelude::*;

use home_invasion::{
    cfg::window_plugin::build_platform_window_plugin,
    components::{
        cam::CamPlugin, game_menu::GameMenuPlugin, house::HousePlugin, items::ItemsPlugin,
        player::PlayerPlugin, sound::SoundPlugin,
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
        ItemsPlugin,
        CamPlugin,
        HousePlugin,
        PlayerPlugin,
        SoundPlugin,
    ));
    app
}

fn main() {
    build_app().run();
}
