use super::*;
use bevy::math::Rect;
use bevy::{prelude::*, transform};
use bevy::window::PrimaryWindow;

// basic enemy struct
#[derive(Component)]

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system((spawn_enemies));
    }
}


pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let start_x: f32 = 13.0;
        let start_y: f32 = 13.0;
        
        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(start_x, start_y, 1.8),
                    texture: assets.load("zombie_still.png"),
                    ..default()
                },
                EnemyPlugin {},
            ));
    }
}