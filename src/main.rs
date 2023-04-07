#![allow(unused)]

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use cauldron::CauldronPlugin;
use ingredient::IngredientPlugin;
use potion::PotionPlugin;
use shelf::ShelfPlugin;

mod cauldron;
mod ingredient;
mod potion;
mod shelf;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // States
        .add_state::<GameState>()
        .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::Next))
        // Plugins
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(CauldronPlugin)
        .add_plugin(IngredientPlugin)
        .add_plugin(ShelfPlugin)
        .add_plugin(PotionPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Next,
}
