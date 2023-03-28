use bevy::{
  prelude::*, 
  sprite::{MaterialMesh2dBundle, collide_aabb::{collide, Collision}},
};
const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PADDLE_SPEED: f32 = 500.0;
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_PADDING: f32 = 10.0;

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, 0.5);

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const WALL_THICKNESS: f32 = 10.0;
const BOTTOM_WALL: f32 = -300.;

pub fn breakout() {
  App::new()
      .add_plugins(DefaultPlugins)
      .insert_resource(Scoreboard { score: 0 })
      .insert_resource(ClearColor(BACKGROUND_COLOR))
      .insert_resource(FixedTime::new_from_secs(TIME_STEP))
      .add_startup_system(setup)
      .add_event::<CollisionEvent>()
      .add_systems(
          (
              check_for_collisions,
              apply_velocity
                  .before(check_for_collisions),
              move_paddle_system
                  .before(check_for_collisions)
                  .after(apply_velocity)
          ).in_schedule(CoreSchedule::FixedUpdate)
      )
      .add_system(update_scoreboard)
      .add_system(bevy::window::close_on_esc)
      .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Resource)]
struct Scoreboard {
  score: usize,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Default)]
struct CollisionEvent;

fn check_for_collisions(
  mut commands: Commands,
  mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
  collider_query: Query<(Entity, &Transform), With<Collider>>,
  mut collision_events: EventWriter<CollisionEvent>,
){
  let (mut ball_velocity, ball_transform) = ball_query.single_mut();
  let ball_size = ball_transform.scale.truncate();

  for (collider_entity, transform) in &collider_query {
      let collision = collide(
          ball_transform.translation,
          ball_size,
          transform.translation,
          transform.scale.truncate(),
      );
      if let Some(collision) = collision {
          collision_events.send_default();
          
          let mut reflect_x = false;
          let mut reflect_y = false;

          // only reflect if the ball's velocity is going in the opposite direction of the
          // collision
          match collision {
              Collision::Left => reflect_x = ball_velocity.x > 0.0,
              Collision::Right => reflect_x = ball_velocity.x < 0.0,
              Collision::Top => reflect_y = ball_velocity.y < 0.0,
              Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
              Collision::Inside => { /* do nothing */ }
          }

          // reflect velocity on the x-axis if we hit something on the x-axis
          if reflect_x {
              ball_velocity.x = -ball_velocity.x;
          }

          // reflect velocity on the y-axis if we hit something on the y-axis
          if reflect_y {
              ball_velocity.y = -ball_velocity.y;
          }
      }
  }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
  for (mut transform, velocity) in &mut query {
      transform.translation.x += velocity.x * TIME_STEP;
      transform.translation.y += velocity.y * TIME_STEP;
  }
}

fn move_paddle_system(
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<&mut Transform, With<Paddle>>,
){
  let mut paddle_transform = query.single_mut();
  let mut direction = 0.0;

  if keyboard_input.pressed(KeyCode::Left) {
      direction -= 1.0;
  }

  if keyboard_input.pressed(KeyCode::Right) {
      direction += 1.0;
  }

  let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * TIME_STEP;
  let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
  let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

  paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
  let mut text = query.single_mut();
  text.sections[1].value = scoreboard.score.to_string();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  commands.spawn(Camera2dBundle::default());

  // Lets add the paddle first
  let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
  commands.spawn(
      (SpriteBundle {
          transform: Transform {
              translation: Vec3::new(0., paddle_y, 0.),
              scale: PADDLE_SIZE,
              ..default()
          },
          sprite: Sprite {
              color: PADDLE_COLOR,
              ..default()
          },
          ..default()
      }, Paddle, Collider),
  );

  // Let's add the scoreboard
  commands.spawn(
      TextBundle::from_sections([
          TextSection::new(
              "Score: ",
              TextStyle {
                  font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                  font_size: SCOREBOARD_FONT_SIZE,
                  color: TEXT_COLOR,
              },
          ),
          TextSection::from_style(TextStyle {
              font: asset_server.load("fonts/FiraMono-Medium.ttf"),
              font_size: SCOREBOARD_FONT_SIZE,
              color: SCORE_COLOR,
          }),
      ])
      .with_style(Style {
          position_type: PositionType::Absolute,
          position: UiRect {
              top: SCOREBOARD_TEXT_PADDING,
              left: SCOREBOARD_TEXT_PADDING,
              ..default()
          },
          ..default()
      }),
  );

  // Ball
  commands.spawn((
      MaterialMesh2dBundle {
          mesh: meshes.add(shape::Circle::default().into()).into(),
          material: materials.add(ColorMaterial::from(BALL_COLOR)),
          transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
          ..default()
      },
      Ball,
      Collider,
      Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED)    ));

}
