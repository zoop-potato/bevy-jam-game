use bevy::prelude::*;

pub struct IngredientPlugin;

impl Plugin for IngredientPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_ingredient_textures);
    }
}

#[derive(Component)]
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

#[derive(Resource)]
pub struct IngredientTextures {
    mushroom: Handle<Image>,
    deerpiss: Handle<Image>,
    toenails: Handle<Image>,
    rabbitpoo: Handle<Image>,
    fishhead: Handle<Image>,
    frogleg: Handle<Image>,
    corpseflower: Handle<Image>,
    birtchbark: Handle<Image>,
}

fn load_ingredient_textures(mut commands: Commands, assets: Res<AssetServer>) {
    let textures = IngredientTextures {
        mushroom: assets.load("mushroom.png"),
        deerpiss: assets.load("deerpiss.png"),
        toenails: assets.load("toenails.png"),
        rabbitpoo: assets.load("rabbitpoo.png"),
        fishhead: assets.load("fishhead.png"),
        frogleg: assets.load("frogleg.png"),
        corpseflower: assets.load("corpseflower.png"),
        birtchbark: assets.load("birtchbark.png"),
    };
    commands.insert_resource(textures);
}

pub fn spawn_ingredient(
    mut commands: Commands,
    textures: Res<IngredientTextures>,
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
