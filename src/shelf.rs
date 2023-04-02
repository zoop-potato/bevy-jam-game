use super::*;
use crate::ingredient::{spawn_ingredient, Ingredient, IngredientTextures};
use bevy::prelude::*;

pub struct ShelfPlugin;

impl Plugin for ShelfPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_shelf)
            .add_system(put_ingredients_on_shelf.in_schedule(OnEnter(GameState::Next)));
    }
}

#[derive(Resource)]
pub struct Shelf {
    entity: Entity,
}

fn setup_shelf(mut commands: Commands, assets: Res<AssetServer>) {
    let id = commands
        .spawn(SpriteBundle {
            texture: assets.load("shelf.png"),
            transform: Transform::from_xyz(-400.0, 230.0, 0.0).with_scale(Vec3::splat(0.6)),
            ..default()
        })
        .id();
    commands.insert_resource(Shelf { entity: id })
}

fn put_ingredients_on_shelf(
    mut commands: Commands,
    textures: Res<IngredientTextures>,
    shelf: Res<Shelf>,
) {
    // Mushroom
    let mushroom = spawn_ingredient(
        &mut commands,
        &textures,
        Ingredient::Mushroom,
        Transform::from_xyz(-240.0, -50.0, 0.1).with_scale(Vec3::splat(0.4)),
    );
    commands.add(AddChild {
        parent: shelf.entity,
        child: mushroom,
    });

    // DeerPiss
    let deerpiss = spawn_ingredient(
        &mut commands,
        &textures,
        Ingredient::DeerPiss,
        Transform::from_xyz(-240.0, 75.0, 0.1).with_scale(Vec3::splat(0.4)),
    );
    commands.add(AddChild {
        parent: shelf.entity,
        child: deerpiss,
    });

    // ToeNails
    let toenails = spawn_ingredient(
        &mut commands,
        &textures,
        Ingredient::ToeNails,
        Transform::from_xyz(65.0, 75.0, 0.1).with_scale(Vec3::splat(0.4)),
    );
    commands.add(AddChild {
        parent: shelf.entity,
        child: toenails,
    });

    // RabbitPoo
    let rabbitpoo = spawn_ingredient(
        &mut commands,
        &textures,
        Ingredient::RabbitPoo,
        Transform::from_xyz(-80.0, 75.0, 0.1).with_scale(Vec3::splat(0.4)),
    );
    commands.add(AddChild {
        parent: shelf.entity,
        child: rabbitpoo,
    });

    // FishHead
    let fishhead = spawn_ingredient(
        &mut commands,
        &textures,
        Ingredient::FishHead,
        Transform::from_xyz(70.0, -50.0, 0.1).with_scale(Vec3::splat(0.4)),
    );
    commands.add(AddChild {
        parent: shelf.entity,
        child: fishhead,
    });

    // FrogLegs
    let froglegs = spawn_ingredient(
        &mut commands,
        &textures,
        Ingredient::FrogLeg,
        Transform::from_xyz(-80.0, -50.0, 0.1).with_scale(Vec3::splat(0.4)),
    );
    commands.add(AddChild {
        parent: shelf.entity,
        child: froglegs,
    });

    // CorpseFlower
    let corpseflower = spawn_ingredient(
        &mut commands,
        &textures,
        Ingredient::CorpseFlower,
        Transform::from_xyz(205.0, 75.0, 0.1).with_scale(Vec3::splat(0.5)),
    );
    commands.add(AddChild {
        parent: shelf.entity,
        child: corpseflower,
    });

    // BirtchBark
    let birtchbark = spawn_ingredient(
        &mut commands,
        &textures,
        Ingredient::BirtchBark,
        Transform::from_xyz(220.0, -55.0, 0.1).with_scale(Vec3::splat(0.35)),
    );
    commands.add(AddChild {
        parent: shelf.entity,
        child: birtchbark,
    });
}
