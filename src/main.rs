#![allow(unused)]

use bevy_kira_audio::prelude::*;
use bevy_kira_audio::prelude::Audio; // Preventing naming conflict between kira_audio and regular bevy (both have Audio) 
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use cauldron::CauldronPlugin;
use ingredient::IngredientPlugin;
use potion::PotionPlugin;
use shelf::ShelfPlugin;
//use audio::AudioPlugin;


mod cauldron;
mod ingredient;
mod potion;
mod shelf;
mod audio;


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
        .add_plugin(AudioPlugin)
        .add_startup_system(setup_camera)
        .add_system(start_background_audio.on_startup())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Simple method to loop the OST on startup with kira_audio ver 0.15.0 
fn start_background_audio(assets: Res<AssetServer>, audio: Res<Audio>) { 
    audio.play(assets.load("etntrack.ogg")).looped();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Next,
}
