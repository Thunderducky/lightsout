use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use illumination_no_no::AppPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

const ENABLE_INSPECTOR: bool = false;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Illumination No No".to_string(),
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }));

    if ENABLE_INSPECTOR {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.add_plugin(AppPlugin).run();
}
