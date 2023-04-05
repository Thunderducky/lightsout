use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{gameplugin, mainmenuplugin, victoryscreenplugin};

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    Paused,
    Victory,
}

// Add a mouse resource?
pub fn lightsout() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_state::<AppState>()
        .add_plugin(gameplugin::GamePlugin)
        .add_plugin(mainmenuplugin::MainMenuPlugin)
        .add_plugin(victoryscreenplugin::VictoryScreenPlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
    // , mut next_state: ResMut<NextState<AppState>>,
) {
    commands.spawn(Camera2dBundle::default());
    // let music = asset_server.load("./audio/Carefree.mp3");
    //audio.play_with_settings(music, PlaybackSettings { repeat: true, volume: 0.05, speed: 1. });
    // next_state.set(AppState::Victory)
}
