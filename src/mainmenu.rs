use bevy::prelude::*;

use crate::loading::FontAssets;
use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>();

        app.add_system(main_menu_setup.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(main_menu_update.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(main_menu_teardown.in_schedule(OnExit(AppState::MainMenu)));
    }
}

#[derive(Component)]
pub struct MainMenuUI;

// Some default UI help
#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> ButtonColors {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}


fn main_menu_setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
    .spawn(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(120.0), Val::Px(50.0)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: button_colors.normal.into(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Play",
            TextStyle {
                font: font_assets.fira_sans_bold.clone(),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));
    }).insert(MainMenuUI);
}
fn main_menu_update(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}
fn main_menu_teardown(
    mut commands: Commands,
    button: Query<Entity, With<MainMenuUI>>
) {
    commands.entity(button.single()).despawn_recursive();
}
