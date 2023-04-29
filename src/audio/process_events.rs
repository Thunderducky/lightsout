// Eventually swap these for domain events?
// Or just keep it generic

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::loading::AudioAssets;

use super::{MusicChannel, ChannelAudioState, GameSfxChannel, AudioEvent, AudioEventData};

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
  // mut music_channel_state: ResMut<ChannelAudioState<MusicChannel>>,
  mut music_channel: Res<AudioChannel<MusicChannel>>,
  mut sfx_channel: Res<AudioChannel<GameSfxChannel>>,
  audio_sources: Res<AudioAssets>
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
                music_channel.play(audio_sources.bg_music.clone());
            }
        }
    }
}

/*

fn play_sound_button<T: Component + Default>(
    channel: Res<AudioChannel<T>>,
    time: Res<Time>,
    mut last_action: ResMut<LastAction>,
    mut channel_state: ResMut<ChannelAudioState<T>>,
    audio_handles: Res<AudioHandles>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), With<PlaySoundButton<T>>>,
) {
    let (interaction, mut color) = interaction_query.single_mut();
    *color = if interaction == &Interaction::Hovered {
        HOVERED_BUTTON.into()
    } else {
        NORMAL_BUTTON.into()
    };
    if interaction == &Interaction::Clicked {
        if !last_action.action(&time) {
            return;
        }
        channel_state.paused = false;
        channel_state.stopped = false;
        channel.play(audio_handles.sound_handle.clone());
    }
}
 */
