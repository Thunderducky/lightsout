use bevy::prelude::*;

use crate::AppState;

use super::TileInfo;

#[derive(Default, Resource)]
pub struct Actions {
    pub grid_selection: Option<(i32, i32)>,
    pub activated: bool,
    pub selected: bool
}

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_system(process_input_commands.in_set(OnUpdate(AppState::Game)));
    }
}

// Gets the mouse position and converts it to a grid position
fn process_input_commands(
    mut action: ResMut<Actions>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    query: Query<(&TileInfo, &Transform)>,
    buttons: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let mut mouse_position = Vec2::ZERO;
    for event in cursor_moved_events.iter() {
        mouse_position = event.position;
        action.selected = false;
    }
    action.activated = buttons.just_pressed(MouseButton::Left);
    
    let camera = camera_q.single();
    let world_point_option = screen_to_world(camera, mouse_position);
    
    action.grid_selection = None;
    
    
    if let Some(world_point) = world_point_option {
        for (tile, transform) in query.iter() {
            if transform_contains_point(&transform, world_point) {
                action.grid_selection = Some((tile.grid_x, tile.grid_y));
                action.selected = true;
            }
        }
    }
}

pub fn screen_to_world(camera_q: (&Camera, &GlobalTransform), screen_position: Vec2) -> Option<Vec2> {
  // get the camera info and transform
  // assuming there is exactly one main camera entity, so query::single() is OK
  let (camera, camera_transform) = camera_q;
  camera
      .viewport_to_world(camera_transform, screen_position)
      .map(|ray| ray.origin.truncate())
}

pub fn transform_contains_point(transform: &Transform, point: Vec2) -> bool {
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