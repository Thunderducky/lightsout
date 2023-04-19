use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
  fn build(&self, app: &mut App) {
    app.add_loading_state(
      LoadingState::new(AppState::Loading).continue_to_state(AppState::MainMenu),
    )
    .add_collection_to_loading_state::<_, FontAssets>(AppState::Loading)
    .add_collection_to_loading_state::<_, AudioAssets>(AppState::Loading)
    ;
  }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans_bold: Handle<Font>,

    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub fira_mono_medium: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/Carefree.mp3")]
    pub bg_music: Handle<AudioSource>,

    #[asset(path = "audio/switch30.ogg")]
    pub click_fx: Handle<AudioSource>,
}