mod loading;
mod mainmenu;
mod audio;
mod game;
mod victory;

use bevy::prelude::*;


use crate::game::GamePlugin;
use crate::mainmenu::MainMenuPlugin;
use crate::loading::LoadingPlugin;
use crate::victory::VictoryPlugin;
use crate::audio::InternalAudioPlugin;



#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    Game,
    Paused,
    Victory,
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Default things to add
        app.add_state::<AppState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(GamePlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(VictoryPlugin)
            .add_system(bevy::window::close_on_esc);

        // Debug things to add
    }
}