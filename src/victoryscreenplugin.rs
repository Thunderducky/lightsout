use bevy::prelude::*;

use crate::lightsout::AppState;

pub struct VictoryScreenPlugin;

#[derive(Component)]
struct VictoryUI;

impl Plugin for VictoryScreenPlugin {
    fn build(&self, app: &mut App) {
      app.add_system(victory_setup.in_schedule(OnEnter(AppState::Victory)))
        .add_system(victory_teardown.in_schedule(OnExit(AppState::Victory)));
    }
}

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
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Victory!", style));
            parent.spawn(TextBundle::from_section("thats it... go home, press Esc", style2));
        });
}

fn victory_teardown(mut commands: Commands, query: Query<Entity, With<VictoryUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}