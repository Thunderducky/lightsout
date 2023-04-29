// Eventually swap these for domain events?
// Or just keep it generic

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::loading::AudioAssets;

use super::{AudioEvent, AudioEventData, ChannelAudioState, GameSfxChannel, MusicChannel};

#[allow(dead_code)]
pub fn keyboard_test(keys: Res<Input<KeyCode>>, mut event_writer: EventWriter<AudioEvent>) {
    if keys.just_pressed(KeyCode::S) {
        // Space was pressed
        event_writer.send(AudioEvent(AudioEventData::PlaySound));
    }
    if keys.just_pressed(KeyCode::M) {
        // Space was pressed
        event_writer.send(AudioEvent(AudioEventData::StartMusic));
    }
}

pub fn process_audio_events(
    mut event_reader: EventReader<AudioEvent>,
    mut music_channel_state: ResMut<ChannelAudioState<MusicChannel>>,
    mut sfx_channel_state: ResMut<ChannelAudioState<GameSfxChannel>>,
    music_channel: Res<AudioChannel<MusicChannel>>,
    sfx_channel: Res<AudioChannel<GameSfxChannel>>,
    audio_sources: Res<AudioAssets>,
) {
    for ev in event_reader.iter() {
        match ev.0 {
            AudioEventData::PlaySound => {
                println!("Play sound");
                sfx_channel.play(audio_sources.click_fx.clone());
            }
            AudioEventData::StartMusic => {
                println!("Start music");
                // If we already have music then don't start it again
                music_channel_state.paused = false;
                music_channel_state.stopped = false;
                music_channel_state.volume = 1.0;
                music_channel_state.loop_started = true;
                music_channel.play(audio_sources.bg_music.clone()).looped();
            }
            AudioEventData::ToggleMute => {
                if music_channel_state.volume > 0.0 {
                    music_channel_state.volume = 0.0;
                    sfx_channel_state.volume = 0.0;
                } else {
                    music_channel_state.volume = 1.0;
                    sfx_channel_state.volume = 1.0;
                }
                sfx_channel.set_volume(sfx_channel_state.volume);
                music_channel.set_volume(music_channel_state.volume);
            }
        }
    }
}
