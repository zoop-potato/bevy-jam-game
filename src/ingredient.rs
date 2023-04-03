use super::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct IngredientPlugin;

impl Plugin for IngredientPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, IngredientTextures>(GameState::Loading)
            .insert_resource(IngredientDragState::default())
            .add_system(apply_gravity.in_set(OnUpdate(GameState::Next)));
    }
}

#[derive(Component, Default)]
pub struct Gravity {
    speed: f32,
}

#[derive(Component, Debug, Copy, Clone)]
pub enum Ingredient {
    Mushroom,
    DeerPiss,
    ToeNails,
    RabbitPoo,
    FishHead,
    FrogLeg,
    CorpseFlower,
    BirtchBark,
}

#[derive(AssetCollection, Resource)]
pub struct IngredientTextures {
    #[asset(path = "mushroom.png")]
    mushroom: Handle<Image>,
    #[asset(path = "deerpiss.png")]
    deerpiss: Handle<Image>,
    #[asset(path = "toenails.png")]
    toenails: Handle<Image>,
    #[asset(path = "rabbitpoo.png")]
    rabbitpoo: Handle<Image>,
    #[asset(path = "fishhead.png")]
    fishhead: Handle<Image>,
    #[asset(path = "froglegs.png")]
    frogleg: Handle<Image>,
    #[asset(path = "corpseflower.png")]
    corpseflower: Handle<Image>,
    #[asset(path = "birtchbark.png")]
    birtchbark: Handle<Image>,
}

#[derive(Resource, Default, PartialEq)]
pub enum IngredientDragState {
    Dragging {
        entity: Entity,
        position: Vec2,
    },
    #[default]
    None,
}

pub fn spawn_ingredient(
    commands: &mut Commands,
    textures: &Res<IngredientTextures>,
    ingredient: Ingredient,
    transform: Transform,
) -> Entity {
    use Ingredient::*;
    let texture = match ingredient {
        Mushroom => textures.mushroom.clone_weak(),
        DeerPiss => textures.deerpiss.clone_weak(),
        ToeNails => textures.toenails.clone_weak(),
        RabbitPoo => textures.rabbitpoo.clone_weak(),
        FishHead => textures.fishhead.clone_weak(),
        FrogLeg => textures.frogleg.clone_weak(),
        CorpseFlower => textures.corpseflower.clone_weak(),
        BirtchBark => textures.birtchbark.clone_weak(),
    };
    commands
        .spawn(SpriteBundle {
            texture: texture,
            transform: transform,
            ..default()
        })
        .insert(ingredient)
        .id()
}

fn apply_gravity(mut objects: Query<(&mut Gravity, &mut Transform)>) {
    for (mut grav, mut transform) in objects.iter_mut() {
        grav.speed -= 0.3;
        transform.translation.y += grav.speed;
    }
}
