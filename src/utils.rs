use bevy::prelude::*;

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