use bevy::prelude::*;

use crate::gameplugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    Paused,
    Victory,
}
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
// Add a mouse resource?
pub fn lightsout() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_state::<AppState>()
        .add_plugin(gameplugin::GamePlugin)
        .add_startup_system(setup)
        .add_system(victory_setup.in_schedule(OnEnter(AppState::Victory)))
        .add_system(victory_teardown.in_schedule(OnExit(AppState::Victory)))
        .add_system(main_menu_setup.in_schedule(OnEnter(AppState::MainMenu)))
        .add_system(main_menu_update.in_set(OnUpdate(AppState::MainMenu)))
        .add_system(main_menu_teardown.in_schedule(OnExit(AppState::MainMenu)))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    commands.spawn(Camera2dBundle::default());
}
#[derive(Component)]
struct MainMenuUI;
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
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
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
    mut next_state: ResMut<NextState<AppState>>,
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
                next_state.set(AppState::Game);

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
}
fn main_menu_teardown(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Victory component
#[derive(Component)]
struct VictoryUI;

fn victory_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let style = TextStyle {
        font: font.clone(),
        font_size: 100.0,
        color: Color::WHITE,
    };
    let style2 = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::GRAY,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            VictoryUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Victory!", style));
            parent.spawn(TextBundle::from_section("thats it... go home", style2));
        });
}

fn victory_teardown(mut commands: Commands, query: Query<Entity, With<VictoryUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
