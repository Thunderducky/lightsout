pub mod actions;
mod puzzle_word_encoder;
mod tile_puzzle;

use self::{actions::Actions, tile_puzzle::TilePuzzle};
use crate::{AppState, audio::{AudioEvent, AudioEventData}};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilePuzzle::new())
            .add_plugin(actions::ActionsPlugin)
            .add_system(game_enter.in_schedule(OnEnter(AppState::Game)))
            .add_system(game_exit.in_schedule(OnExit(AppState::Game)))
            .add_system(process_actions.in_set(OnUpdate(AppState::Game)));
    }
}

fn game_enter(mut commands: Commands, mut tile_puzzle: ResMut<TilePuzzle>, mut event_writer: EventWriter<AudioEvent>,) {
    // Initialize a level
    event_writer.send(AudioEvent(AudioEventData::StartMusic));
    tile_puzzle.generate_random_puzzle();

    for (index, value) in tile_puzzle.tile_values.iter().enumerate() {
        let x = (index % 5) as i32;
        let y = (index / 5) as i32;
        commands.spawn(build_tile_setup(
            (x - 2) as f32 * 60.,
            (y - 2) as f32 * 60.,
            x,
            y as i32,
            *value,
        ));
    }
}
fn process_actions(
    action: Res<Actions>,
    mut state: ResMut<NextState<AppState>>,
    mut event_writer: EventWriter<AudioEvent>,
    mut tile_puzzle: ResMut<TilePuzzle>,
    mut tiles: Query<(&mut TileInfo, &mut Sprite)>,
) {
    // Handle Clicks
    if action.activated {
        if let Some((x, y)) = action.grid_selection {
            tile_puzzle.toggle_tile(x, y);
            event_writer.send(AudioEvent(AudioEventData::PlaySound));
            // Don't need to do this all the time
            for (tile_info, mut sprite) in tiles.iter_mut() {
                let index = (tile_info.grid_y * tile_puzzle.width + tile_info.grid_x) as usize;
                sprite.color = match tile_puzzle.tile_values[index] {
                    true => ON_BUTTON,
                    false => OFF_BUTTON,
                };
            }
        }
    }
    if tile_puzzle.is_solved() {
        info!("You win!");
        state.set(AppState::Victory);
    }
    // check for victory
}

fn game_exit(mut commands: Commands, query: Query<Entity, With<TileInfo>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct TileInfo {
    pub grid_x: i32,
    pub grid_y: i32,
    pub light_on: bool,
}
// TODO: Convert this to a resource, also, convert this in the colorize plugin
const OFF_BUTTON: Color = Color::hsl(195., 1., 0.2);
const ON_BUTTON: Color = Color::hsl(195., 1., 0.7);

fn build_tile_setup(
    x: f32,
    y: f32,
    grid_x: i32,
    grid_y: i32,
    light_on: bool,
) -> (SpriteBundle, TileInfo) {
    (
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                scale: Vec3::new(50., 50., 0.),
                ..default()
            },
            sprite: Sprite {
                color: match light_on {
                    true => ON_BUTTON,
                    false => OFF_BUTTON,
                },
                ..default()
            },
            ..default()
        },
        TileInfo {
            grid_x,
            grid_y,
            light_on,
        },
    )
}
