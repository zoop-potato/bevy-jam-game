use bevy::prelude::shape::Box;
use bevy::prelude::*;

pub struct CauldronPlugin;

impl Plugin for CauldronPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_cauldron);
    }
}

#[derive(Resource)]
pub struct Cauldron {
    contains: Vec<Entity>,
    max_ingredients: usize,
    bobbing_box: Box,
    entity: Entity,
}

fn setup_cauldron(mut commands: Commands, assets: Res<AssetServer>) {
    let id = commands
        .spawn(SpriteBundle {
            texture: assets.load("ol_bethy_static.png"),
            transform: Transform::from_xyz(-410.0, -130.0, 0.0).with_scale(Vec3 {
                x: 2.0,
                y: 2.0,
                z: 0.0,
            }),
            ..default()
        })
        .id();

    commands.insert_resource(Cauldron {
        contains: vec![],
        max_ingredients: 6,
        bobbing_box: Box::from_corners(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        ),
        entity: id,
    });
}
