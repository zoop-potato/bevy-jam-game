use super::*;
use bevy::math::Rect;
use bevy::{prelude::*, transform};
use bevy::window::PrimaryWindow;

#[derive(Component)]

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemies)
           .add_startup_system(player_hitbox)
           .add_system(enemy_movement);
    }
}


// temp sprite, you can remove it if you want. 
#[derive(Component)]
pub struct Player {
}


pub fn player_hitbox(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle { 
            transform: Transform::from_xyz(-410.0, -130.0, 0.3).with_scale(Vec3 {
                x: 2.0,
                y: 2.0,
                z: 0.0,
            }),
            texture: assets.load("ol_bethy_static.png"),
            ..default()
        },
        Player {},
    ));
}

// basic enemy struct
#[derive(Component)]

pub struct Enemy{
    pub direction: Vec2, 
}

// spawner, if you want to create an interval spawner, do so in here i guess?

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let start_x: f32 = 460.0;
        let start_y: f32 = -90.0;
        
        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(start_x, start_y, 1.8),
                    texture: assets.load("zombie_still.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(-1.0, 0.0).normalize(),
                },
            ));
    }
}

// enemy movement, to adjust speed change the pub const ENEMY_SPEED in main.rs

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction: Vec3 = Vec3::new(enemy.direction.x, 0.0, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
      //  println!("{}", transform.translation.x.to_string());
    }
}

   