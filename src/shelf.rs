use bevy::prelude::*;

struct ShelfPlugin;

impl Plugin for ShelfPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Resource)]
struct Shelf {}
