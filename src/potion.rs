use super::*;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use cauldron::{CatchEvent, Cauldron};
use ingredient::*;
use shelf::{screen_to_world, ClickBox, ClickEvent};

pub struct PotionPlugin;

impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, PotionTexture>(GameState::Loading)
            .add_system(craft_potion.in_set(OnUpdate(GameState::Next)))
            .add_system(check_potion_clicks.in_set(OnUpdate(GameState::Next)));
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
            transform: Transform::from_xyz(0.0, 0.0, 0.1).with_scale(Vec3::splat(0.4)),
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

fn check_potion_clicks(
    mut commands: Commands,
    mut click_reader: EventReader<ClickEvent>,
    mut drag_state: ResMut<EntityDragState>,
    potion: Query<(&PotionEffect, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = cameras.get_single().unwrap();
    let window = windows.get_single().unwrap();
    for event in click_reader.iter() {
        if potion.contains(event.0) {
            let mouse = screen_to_world(
                Vec2::new(window.width(), window.height()),
                window.cursor_position().unwrap(),
                camera,
                camera_transform,
            );
            *drag_state.as_mut() = EntityDragState::Dragging {
                entity: event.0,
                position: mouse,
            }
        }
    }
}
