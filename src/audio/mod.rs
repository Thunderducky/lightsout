use std::marker::PhantomData;

use crate::{loading::AudioAssets, AppState};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use self::process_events::*;

mod process_events;

pub struct InternalAudioPlugin;

// #[derive(Resource)]
// struct BgMusicAudio {
//     volume: f64,
//     value: Option<Handle<AudioInstance>>
// }

// #[derive(Resource)]
// struct SoundFxAudio {
//     volume: f64,
//     value: Option<Handle<AudioInstance>>
// }
// main_menu_setup.in_schedule(OnEnter(AppState::MainMenu)
// Set Audio Channels
#[derive(Resource, Component, Default, Clone)]
pub struct MusicChannel;
#[derive(Resource, Component, Default, Clone)]
pub struct GameSfxChannel;
// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system(init_audio_channels.on_startup())
            .add_audio_channel::<MusicChannel>()
            .add_audio_channel::<GameSfxChannel>()
            .add_event::<AudioEvent>()
            .add_system(process_audio_events.run_if(resource_exists::<AudioAssets>()))
            ;

        // app.add_system(keyboard_test);
    }
}

fn init_audio_channels(mut commands: Commands) {
    commands.insert_resource(ChannelAudioState::<MusicChannel>::default());
    commands.insert_resource(ChannelAudioState::<GameSfxChannel>::default());
}

#[derive(Resource)]
pub struct ChannelAudioState<T> {
    stopped: bool,
    paused: bool,
    loop_started: bool,
    volume: f64,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for ChannelAudioState<T> {
    fn default() -> Self {
        ChannelAudioState { 
            stopped: true, 
            paused: false, 
            loop_started: false, 
            volume: 1.0, 
            _marker: PhantomData::<T>::default() 
        }
    }
}

pub enum AudioEventData {
    PlaySound,
    StartMusic,
}

pub struct AudioEvent(pub AudioEventData);


// pub enum AudioSettingChange {
//     ToggleMute,
//     PlaySound
// }
// pub enum AudioType {
//     Music,
//     Sound
// }

// pub struct AudioEvent {
//     change: AudioSettingChange,
//     audio_type: AudioType,
// }

// fn process_audio_events(
//     mut play_sound_events: EventReader<AudioEvent>,
//     mut bg_handle: ResMut<BgMusicAudio>,
//     mut sfx_handle: ResMut<SoundFxAudio>,
//     mut audio_instances: ResMut<Assets<AudioInstance>>,
// ) {
//     for event in play_sound_events.iter() {
//         match event.audio_type {
//             AudioType::Music => {
//                 if let Some(bg_music) = &mut bg_handle.value {
//                     if let Some(instance) = audio_instances.get_mut(&bg_music) {
//                         match event.change {
//                             AudioSettingChange::ToggleMute => {
//                                 // Toggle mute on the music
//                                 if bg_handle.volume > 0. {
//                                     bg_handle.volume = 0.;
//                                 } else {
//                                     bg_handle.volume = 0.1;
//                                 }
//                                 instance.set_volume(bg_handle.volume, AudioTween::default());
//                             },
//                             _ => {}
//                         }
//                     }
//                 }
//             },
//             AudioType::Sound => {
//                 println!("Play sound1");
//                 if let Some(sfx) = &mut sfx_handle.value {
//                     println!("Play sound2");
//                     if let Some(instance) = audio_instances.get_mut(&sfx) {
//                         match event.change {
//                             AudioSettingChange::PlaySound => {
//                                 println!("Play sound3");
//                                 // Toggle mute on the music
//                                 instance.seek_to(0.);
//                                 sfx_handle.volume = 1.0;
//                                 instance.set_volume(sfx_handle.volume, AudioTween::default());
//                                 instance.resume(AudioTween::default());
//                             },
//                             _ => {}
//                         }
//                     }
//                 }
//             }
//         };
//     }
// }

// fn start_audio(
//     // mut commands: Commands,
//     audio_assets: Res<AudioAssets>,
//     audio: Res<Audio>,
//     mut bg_music: ResMut<BgMusicAudio>,
//     mut sfx: ResMut<SoundFxAudio>,
// ) {
//     bg_music.volume = 0.0;
//     let music_handle = audio
//         .play(audio_assets.bg_music.clone())
//         .looped()
//         .with_volume(bg_music.volume)
//         .handle();

//     bg_music.value = Some(music_handle.clone());

//     sfx.volume = 0.1;
//     let sound_handle = audio
//         .play(audio_assets.click_fx.clone())
//         .with_volume(0.)
//         .handle();

//     sfx.value = Some(sound_handle.clone());
//     //commands.insert_resource(BgMusicAudio(music_handle));
// }
