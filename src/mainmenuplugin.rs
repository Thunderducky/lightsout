use std::time::Duration;

use bevy::prelude::*;

use crate::lightsout::AppState;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MainMenuPlugin;

#[derive(Component)]
struct MainMenuUI;

#[derive(Resource)]
struct MainMenuTransitionTimer {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
    started: bool,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MainMenuTransitionTimer {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
            started: false,
        })
        .add_system(main_menu_setup.in_schedule(OnEnter(AppState::MainMenu)))
        .add_system(main_menu_update.in_set(OnUpdate(AppState::MainMenu)))
        .add_system(main_menu_teardown.in_schedule(OnExit(AppState::MainMenu)));
    }
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(170.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
fn main_menu_update(
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
    mut countdown: ResMut<MainMenuTransitionTimer>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Lights out".to_string();
                *color = PRESSED_BUTTON.into();
                if !countdown.started {
                    countdown.started = true;
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = "Lights out".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Lights out".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
    if countdown.started {
        countdown.timer.tick(time.delta());
        if countdown.timer.finished() {
            next_state.set(AppState::Game);
        }
        // TODO: adjust opacity
    }
}
fn main_menu_teardown(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Let's make a transition timer
