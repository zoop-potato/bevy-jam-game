use super::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use cauldron::{CatchEvent, Cauldron};
use ingredient::*;
use shelf::ClickBox;

pub struct PotionPlugin;

impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, PotionTexture>(GameState::Loading)
            .add_system(craft_potion.in_set(OnUpdate(GameState::Next)));
    }
}

#[derive(Component)]
enum PotionEffect {
    JumpForward,
    StinkyFirst,
    FishWall,
    BounceBack,
    DOTSlow,
    Slip,
    ZombieWall,
    SlipChoke,
    BuffDef,
    PosionDOT,
    PosionBurst,
    HighZomb,
    SuperHigh,
}

fn craft_potion(
    mut commands: Commands,
    mut cauldron: ResMut<Cauldron>,
    mut potion: Option<ResMut<CurrentPotion>>,
    ingredients: Query<(Entity, &Ingredient)>,
    potion_texture: Res<PotionTexture>,
    mut events: EventReader<CatchEvent>,
) {
    for event in &mut events {
        let first = event.0;
        let second = event.1;
        let f: Option<&Ingredient> = if first.is_some() {
            ingredients.get_component::<Ingredient>(first.unwrap()).ok()
        } else {
            None
        };
        let s: Option<&Ingredient> = if second.is_some() {
            ingredients
                .get_component::<Ingredient>(second.unwrap())
                .ok()
        } else {
            None
        };
        let effect = map_ingredients_to_effect(f, s);
        spawn_potion(&mut commands, &potion_texture, effect)
    }
}

#[derive(AssetCollection, Resource)]
struct PotionTexture {
    #[asset(path = "potion.png")]
    potion: Handle<Image>,
}

#[derive(Resource)]
struct CurrentPotion {
    potion: Entity,
}

fn spawn_potion(commands: &mut Commands, texture: &Res<PotionTexture>, effect: PotionEffect) {
    commands
        .spawn(effect)
        .insert(SpriteBundle {
            transform: Transform::default().with_scale(Vec3::splat(0.4)),
            texture: texture.potion.clone(),
            ..default()
        })
        .insert(ClickBox {
            topleft: Vec2 { x: -24.0, y: 34.0 },
            bottomright: Vec2 { x: 24.0, y: -30.0 },
        });
}

fn map_ingredients_to_effect(
    first: Option<&Ingredient>,
    second: Option<&Ingredient>,
) -> PotionEffect {
    use Ingredient::*;
    use PotionEffect::*;
    let first = first.unwrap_or(&Ingredient::CorpseFlower);
    if second.is_none() {
        return match first {
            BirtchBark => BuffDef,
            RabbitPoo => ZombieWall,
            ToeNails => DOTSlow,
            DeerPiss => Slip,
            FishHead => SlipChoke,
            CorpseFlower => PosionBurst,
            Mushroom => HighZomb,
            FrogLeg => BounceBack,
        };
    }
    let second = second.unwrap();
    return match first {
        BirtchBark => {
            todo!()
        }
        _ => todo!(),
    };
}
