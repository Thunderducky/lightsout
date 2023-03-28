use std::os::unix::process;

use bevy::{
    ecs::world,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    winit::WinitSettings,
};

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

const OFF_BUTTON: Color = Color::hsl(195., 1., 0.2);
const ON_BUTTON: Color = Color::hsl(195., 1., 0.7);
const HOVERED_BUTTON: Color = Color::hsl(195., 0.5, 0.5);
const PRESSED_BUTTON: Color = Color::hsl(195., 0.8, 0.9);

// Add a mouse resource?

pub fn lightsout() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClickedTile { tile_position: None })
        .insert_resource(MouseSettings {
            mouse_position: Vec2::new(0., 0.),
            mouse_pressed: false,
            new_mouse_release: false,
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_systems((
            process_mouse_events_system,
            process_mouse_commands.after(process_mouse_events_system),
            propogate.after(process_mouse_commands),
        ))
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Tile {
    x: i32,
    y: i32,
    on: bool,
}

#[derive(Resource)]
struct MouseSettings {
    mouse_position: Vec2,
    mouse_pressed: bool,
    new_mouse_release: bool,
}

#[derive(Resource)]
struct ClickedTile {
    tile_position: Option<(i32, i32)>,
}

fn build_tile_setup(x: f32, y: f32, index_x: i32, index_y: i32) -> (SpriteBundle, Tile) {
    (
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                scale: Vec3::new(50., 50., 0.),
                ..default()
            },
            sprite: Sprite {
                color: OFF_BUTTON,
                ..default()
            },
            ..default()
        },
        Tile {
            x: index_x,
            y: index_y,
            on: false,
        },
    )
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for x in 0..5 {
        for y in 0..5 {
            commands.spawn(build_tile_setup(
                (x - 2) as f32 * 60.,
                (y - 2) as f32 * 60.,
                x,
                y,
            ));
        }
    }
}

// Todo: Detect mouse hover and clicks
fn process_mouse_events_system(
    mut mouse_settings: ResMut<MouseSettings>,
    buttons: Res<Input<MouseButton>>,
    // mut mouse_button_input_events: EventReader<MouseButtonInput>,
    // mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    // mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in cursor_moved_events.iter() {
        // info!("{:?}", event);
        mouse_settings.mouse_position = event.position;
    }
    mouse_settings.mouse_pressed = false;
    if buttons.pressed(MouseButton::Left) {
        // Left button is pressed
        mouse_settings.mouse_pressed = true;
    }
    mouse_settings.new_mouse_release = false;
    if buttons.just_released(MouseButton::Left) {
        // Left button was pressed
        mouse_settings.new_mouse_release = true;
    }
}

fn transform_contains_point(transform: &Transform, point: Vec2) -> bool {
    let translation = transform.translation;
    let scale = transform.scale;
    let x = translation.x;
    let y = translation.y;
    let w = scale.x;
    let h = scale.y;
    let left = x - w / 2.;
    let right = x + w / 2.;
    let top = y + h / 2.;
    let bottom = y - h / 2.;
    point.x >= left && point.x <= right && point.y >= bottom && point.y <= top
}

fn is_tile_neighbor(tile: &(i32, i32), other: &Tile) -> bool {
    let x = tile.0;
    let y = tile.1;
    let other_x = other.x;
    let other_y = other.y;
    (x == other_x && (y == other_y + 1 || y == other_y - 1))
        || (y == other_y && (x == other_x + 1 || x == other_x - 1))
}

fn process_mouse_commands(
    mouse_settings: Res<MouseSettings>,
    mut clicked_tile: ResMut<ClickedTile>,
    camera_q: Query<(&Camera, &GlobalTransform)>,

    mut query: Query<(&mut Tile, &Transform, &mut Sprite)>,
) {
    let camera = camera_q.single();

    let world_point_option = screen_to_world(camera, mouse_settings.mouse_position);

    clicked_tile.tile_position = None;
    {
        for (mut tile, transform, mut sprite) in query.iter_mut() {
            if let Some(world_point) = world_point_option {
                if transform_contains_point(&transform, world_point) {
                    if mouse_settings.new_mouse_release {
                        clicked_tile.tile_position = Some((tile.x, tile.y));
                    }

                    if mouse_settings.mouse_pressed {
                        sprite.color = PRESSED_BUTTON;
                    } else {
                        sprite.color = HOVERED_BUTTON;
                    }
                } else {
                    sprite.color = if tile.on { ON_BUTTON } else { OFF_BUTTON };
                }
            } else {
                sprite.color = if tile.on { ON_BUTTON } else { OFF_BUTTON };
            }
        }
    }
}

fn propogate(clicked_tile: ResMut<ClickedTile>, mut query: Query<(&mut Tile, &mut Sprite)>) {
    if let Some(mut tile) = clicked_tile.tile_position {
        // Todo, do a better job on this
        for (mut other, mut sprite) in query.iter_mut() {
            if (tile.0 == other.x && tile.1 == other.y) || is_tile_neighbor(&tile, &other) {
              other.on = !other.on;
            }
        }
    }
}

fn screen_to_world(camera_q: (&Camera, &GlobalTransform), screen_position: Vec2) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q;
    camera
        .viewport_to_world(camera_transform, screen_position)
        .map(|ray| ray.origin.truncate())
}
