use super::*;
use crate::ingredient::{Gravity, Ingredient};
use bevy::math::Rect;
use bevy::{prelude::*, transform};
pub struct CauldronPlugin;

impl Plugin for CauldronPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_cauldron)
            .add_event::<CatchEvent>()
            .add_system(catch_ingredients.in_set(OnUpdate(GameState::Next)));
    }
}

#[derive(Resource)]
pub struct Cauldron {
    contains: Vec<Entity>,
    max_ingredients: usize,
    bobbing_box: Rect,
    entity: Entity,
}

fn setup_cauldron(mut commands: Commands, assets: Res<AssetServer>) {
    let pos = Transform::from_xyz(-410.0, -130.0, 1.0).with_scale(Vec3 {
        x: 2.0,
        y: 2.0,
        z: 0.0,
    });
    let id = commands
        .spawn(SpriteBundle {
            texture: assets.load("ol_bethy_static.png"),
            transform: pos.clone(),
            ..default()
        })
        .id();
    let pos = pos.translation.truncate();
    commands.insert_resource(Cauldron {
        contains: vec![],
        max_ingredients: 2,
        bobbing_box: Rect::from_corners(
            Vec2 { x: -100.0, y: 50.0 } + pos,
            Vec2 { x: 100.0, y: -20.0 } + pos,
        ),
        entity: id,
    });
}

pub struct CatchEvent(pub Option<Entity>, pub Option<Entity>);

fn catch_ingredients(
    mut commands: Commands,
    mut cauldron: ResMut<Cauldron>,
    ingredients: Query<(Entity, &Transform), (With<Ingredient>, With<Gravity>)>,
    mut sender: EventWriter<CatchEvent>,
) {
    let cauldron = cauldron.as_mut();
    for (entity, transform) in ingredients.iter() {
        if cauldron
            .bobbing_box
            .contains(transform.translation.truncate())
        {
            cauldron.contains.push(entity);
            commands.get_entity(entity).unwrap().remove::<Gravity>();
            // Send CatchEvent
            sender.send(CatchEvent(
                cauldron.contains.get(0).copied(),
                cauldron.contains.get(1).copied(),
            ));
        }
    }
}
