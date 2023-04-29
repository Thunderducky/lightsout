use crate::loading::AudioAssets;
use crate::AppState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

#[derive(Resource)]
struct BgMusicAudio {
    volume: f64,
    value: Option<Handle<AudioInstance>>
}

#[derive(Resource)]
struct SoundFxAudio(Handle<AudioInstance>);

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BgMusicAudio{ volume: 0., value: None })
            .add_plugin(AudioPlugin)
            // Don't start the game audio until we actually start the game
            // This prevents the audio from being blocked in the web version
            .add_system(start_audio.in_schedule(OnEnter(AppState::Game)))
            .add_system(keyboard_test_system)
            ;

        // Add event listener
        app.add_event::<AudioEvent>()
            .add_system(process_audio_events);
    }
}

pub enum AudioSettingChange {
    ToggleMute,
}
pub enum AudioType {
    Music,
}

struct AudioEvent {
    change: AudioSettingChange,
    audio_type: AudioType,
}

fn keyboard_test_system(keys: Res<Input<KeyCode>>, mut audio_event: EventWriter<AudioEvent>) {
    if keys.just_pressed(KeyCode::M) {
        println!("M pressed");
        let event = AudioEvent {
            change: AudioSettingChange::ToggleMute,
            audio_type: AudioType::Music,
        };
        audio_event.send(event);
    }
}

fn process_audio_events(
    mut play_sound_events: EventReader<AudioEvent>,
    mut bg_handle: ResMut<BgMusicAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    for event in play_sound_events.iter() {
        match event.audio_type {
            AudioType::Music => {
                if let Some(bg_music) = &mut bg_handle.value {
                    if let Some(instance) = audio_instances.get_mut(&bg_music) {
                        match event.change {
                            AudioSettingChange::ToggleMute => {
                                // Toggle mute on the music
                                if bg_handle.volume > 0. {
                                    bg_handle.volume = 0.;
                                } else {
                                    bg_handle.volume = 0.1;
                                }
                                instance.set_volume(bg_handle.volume, AudioTween::default());
                            }

                        }
                    }
                }
            }
        };
    }
}

fn start_audio(
    // mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut bg_music: ResMut<BgMusicAudio>,
) {
    let music_handle = audio
        .play(audio_assets.bg_music.clone())
        .looped()
        .with_volume(0.1)
        .handle();

    bg_music.volume = 0.1;
    bg_music.value = Some(music_handle.clone());

    //commands.insert_resource(BgMusicAudio(music_handle));
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
