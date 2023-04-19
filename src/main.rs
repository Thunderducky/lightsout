use bevy::prelude::*;
use illumination_no_no::AppPlugin;

// mod lightsout;
// mod utils;
// mod gameplugin;
// mod mainmenuplugin;
// mod victoryscreenplugin;
const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Illumination No No".to_string(),
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AppPlugin)
        .run();
}
