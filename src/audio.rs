use crate::loading::AudioAssets;
use crate::AppState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system(start_audio.in_schedule(OnEnter(AppState::Game)))
            ;
    }
}
#[derive(Resource)]
struct BgMusicAudio(Handle<AudioInstance>);

#[derive(Resource)]
struct SoundFxAudio(Handle<AudioInstance>);


fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
  let music_handle = audio
      .play(audio_assets.bg_music.clone())
      .looped()
      .with_volume(0.1)
      .handle();
    commands.insert_resource(BgMusicAudio(music_handle));
}

// Todo: Events and Systems to control the audio
// Create sound "labels" so the specific sounds can be controlled / mixed / figured out
/*
    Events:
        SetMusicVolume(f32)
        SetSoundFxVolume(f32)
        StartMusic
        StopMusic
        PauseMusic
        ResumeMusic
        PlaySoundFx
        StopSoundFx
 */