use bevy::prelude::*;

use crate::{AppState, gameui};


pub struct VictoryPlugin;

#[derive(Component)]
struct VictoryUI;

impl Plugin for VictoryPlugin {
    fn build(&self, app: &mut App) {
      app.add_system(victory_setup.in_schedule(OnEnter(AppState::Victory)))
        .add_system(victory_teardown.in_schedule(OnExit(AppState::Victory)));
    }
}

fn victory_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: gameui::styles::container::frame(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Victory!", gameui::styles::headers::h1(font.clone())));
            parent.spawn(TextBundle::from_section("thats it... go home, press Esc", gameui::styles::headers::h2(font.clone())));
        });
}

fn victory_teardown(mut commands: Commands, query: Query<Entity, With<VictoryUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}