use super::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use cauldron::Cauldron;
use ingredient::*;

struct PotionPlugin;

impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, PotionTexture>(GameState::Loading)
            .add_system(craft_potion.in_set(GameState::Next));
    }
}

#[derive(Component)]
enum PotionEffect {
    JumpForward,
    StinkyFirst,
    FishWall,
    BounceBack,
    PosionSlow,
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
    cauldron: ResMut<Cauldron>,
    ingredients: Query<(Entity, &Ingredient)>,
    potion_texture: Res<PotionTexture>,
) {
    todo!()
}

#[derive(AssetCollection, Resource)]
struct PotionTexture {
    #[asset(path = "potion.png")]
    potion: Handle<Image>,
}
