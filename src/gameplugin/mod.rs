use bevy::prelude::*;
use crate::{utils, lightsout::AppState};
mod tile_checker;
const OFF_BUTTON: Color = Color::hsl(195., 1., 0.2);
const ON_BUTTON: Color = Color::hsl(195., 1., 0.7);
const HOVERED_BUTTON: Color = Color::hsl(195., 0.5, 0.5);
const PRESSED_BUTTON: Color = Color::hsl(195., 0.8, 0.9);

mod tile_solver;

pub struct GamePlugin;

#[derive(Component)]
pub struct InGame;

#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub on: bool,
}

#[derive(Resource)]
pub struct MouseSettings {
    pub mouse_position: Vec2,
    pub mouse_pressed: bool,
    pub new_mouse_release: bool,
}

#[derive(Resource)]
struct SelectedTile {
    tile_position: Option<(i32, i32)>,
    clicked: bool,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseSettings {
                mouse_position: Vec2::new(0., 0.),
                mouse_pressed: false,
                new_mouse_release: false,
            })
            .insert_resource(SelectedTile {
                tile_position: None,
                clicked: false,
            })
            .add_system(initialize_level.in_schedule(OnEnter(AppState::Game)))
            .add_system(teardown_level.in_schedule(OnExit(AppState::Game)))
            .add_systems((
                process_mouse_events_system,
                process_mouse_commands,
                propogate,
                color_tiles,
                victory_check,
            ).in_set(OnUpdate(AppState::Game)));
    }
}

fn initialize_level(mut commands: Commands) {
    let mut solver = tile_solver::TileSolver::generate_random_puzzle();
    for (index, val) in solver.tile_layout.iter().enumerate() {
        let x = (index % 5) as i32;
        let y = (index / 5) as i32;
        commands.spawn(build_tile_setup(
            (x - 2) as f32 * 60.,
            (y - 2) as f32 * 60.,
            x,
            y as i32,
            *val == 1
        ));
    }
    let result = solver.solve();
    info!("Solution: {:?}", result);
}
// We're going to use this later to allow us to play again or maybe to do a race with levels?
#[allow(dead_code)]
fn shuffle(mut tile_query: Query<&mut Tile>) {
    let mut checker = tile_checker::TileChecker::new();
    for mut tile in tile_query.iter_mut() {
        tile.on = checker.check(rand::random());
    }
}

fn teardown_level(mut commands: Commands, query: Query<Entity, With<InGame>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn build_tile_setup(x: f32, y: f32, index_x: i32, index_y: i32, on:bool) -> (SpriteBundle, Tile, InGame) {
    (
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                scale: Vec3::new(50., 50., 0.),
                ..default()
            },
            sprite: Sprite {
                color: match on {
                    true => ON_BUTTON,
                    false => OFF_BUTTON,
                },
                ..default()
            },
            ..default()
        },
        Tile {
            x: index_x,
            y: index_y,
            on,
        },
        InGame,
    )
}

fn process_mouse_commands(
    mouse_settings: Res<MouseSettings>,
    mut selected_tile: ResMut<SelectedTile>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    query: Query<(&Tile, &Transform)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    let sound = asset_server.load("./audio/switch30.ogg");
    
    let camera = camera_q.single();
    let world_point_option = utils::screen_to_world(camera, mouse_settings.mouse_position);
    
    selected_tile.clicked = false;
    selected_tile.tile_position = None;
    
    if let Some(world_point) = world_point_option {
        for (tile, transform) in query.iter() {
            if utils::transform_contains_point(&transform, world_point) {
                selected_tile.tile_position = Some((tile.x, tile.y));
                if mouse_settings.new_mouse_release {
                    selected_tile.clicked = true;
                    audio.play(sound.clone());
                }
            }
        }
    }
}

fn color_tiles(selected_tile: Res<SelectedTile>, mut query: Query<(&Tile, &mut Sprite)>) {
    for (other, mut sprite) in query.iter_mut() {
        if let Some((x,y)) = selected_tile.tile_position {
            if x == other.x && y == other.y {
                if selected_tile.clicked {
                    sprite.color = PRESSED_BUTTON;
                } else {
                    sprite.color = HOVERED_BUTTON;
                }
            } else if other.on {
                sprite.color = ON_BUTTON;
            } else {
                sprite.color = OFF_BUTTON;
            }
        }
    }
}

fn propogate(clicked_tile: ResMut<SelectedTile>, mut query: Query<&mut Tile>) {
    if clicked_tile.clicked {
        if let Some(tile) = clicked_tile.tile_position {
            for mut other in query.iter_mut() {
                if (tile.0 == other.x && tile.1 == other.y) || is_tile_neighbor(&tile, &other) {
                    other.on = !other.on;
                }
            }
        }
    }
}

fn victory_check(
    query: Query<&mut Tile>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut any_on = false;
    for other in query.iter() {
        any_on = any_on || other.on;
    }
    if !any_on {
        info!("Victory!");
        next_state.set(AppState::Victory);
    }
}

fn process_mouse_events_system(
    mut mouse_settings: ResMut<MouseSettings>,
    buttons: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for event in cursor_moved_events.iter() {
        mouse_settings.mouse_position = event.position;
    }

    mouse_settings.mouse_pressed = buttons.pressed(MouseButton::Left);
    mouse_settings.new_mouse_release = buttons.just_released(MouseButton::Left);
}

fn is_tile_neighbor(tile: &(i32, i32), other: &Tile) -> bool {
    let x = tile.0;
    let y = tile.1;
    let other_x = other.x;
    let other_y = other.y;
    (x == other_x && (y == other_y + 1 || y == other_y - 1))
        || (y == other_y && (x == other_x + 1 || x == other_x - 1))
}
