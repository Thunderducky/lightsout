use std::marker::PhantomData;

use crate::{loading::AudioAssets};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use self::process_events::*;

mod process_events;

pub struct InternalAudioPlugin;

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
    ToggleMute
}

pub struct AudioEvent(pub AudioEventData);
