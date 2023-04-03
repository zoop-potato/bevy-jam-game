use super::*;
use crate::ingredient::{
    spawn_ingredient, Gravity, Ingredient, IngredientDragState, IngredientTextures,
};
use bevy::{transform, window::PrimaryWindow};

pub struct ShelfPlugin;

impl Plugin for ShelfPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_shelf)
            .add_event::<ClickEvent>()
            .add_system(put_ingredients_on_shelf.in_schedule(OnEnter(GameState::Next)))
            .add_systems(
                (update_click_boxes, check_clicks)
                    .chain()
                    .in_set(OnUpdate(GameState::Next)),
            )
            .add_system(drop_ingredient.in_set(OnUpdate(GameState::Next)))
            .add_system(drag_item.in_set(OnUpdate(GameState::Next)));
    }
}

#[derive(Resource)]
pub struct Shelf {
    entity: Entity,
}

#[derive(Component, Clone, Copy)]
pub struct ClickBox {
    topleft: Vec2,
    bottomright: Vec2,
}

pub struct ClickEvent(Entity);

fn update_click_boxes(
    window: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_button: Res<Input<MouseButton>>,
    boxes: Query<(Entity, &GlobalTransform, &ClickBox)>,
    mut click_sender: EventWriter<ClickEvent>,
) {
    // skip if mouse is not just pressed
    if mouse_button.just_pressed(MouseButton::Left) {
        //get camera or fail
        let (camera, camera_transform) = cameras.get_single().unwrap();
        // skip if no prime window
        if let Ok(prime_window) = window.get_single() {
            if let Some(cursor_pos) = prime_window.cursor_position() {
                let world_cursor = screen_to_world(
                    Vec2::new(prime_window.width(), prime_window.height()),
                    cursor_pos,
                    camera,
                    camera_transform,
                );
                for (entity, transform, bx) in boxes.iter() {
                    let transform = transform.compute_transform().translation;
                    let (max_x, min_x) =
                        (transform.x + bx.bottomright.x, transform.x + bx.topleft.x);
                    let (max_y, min_y) =
                        (transform.y + bx.topleft.y, transform.y + bx.bottomright.y);
                    let (x, y) = (world_cursor.x, world_cursor.y);
                    if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                        click_sender.send(ClickEvent(entity));
                        println!("Clicked {:?}", entity);
                        return;
                    }
                }
            }
        }
    }
}

fn check_clicks(
    mut commands: Commands,
    mut click_reader: EventReader<ClickEvent>,
    mut drag_state: ResMut<IngredientDragState>,
    textures: Res<IngredientTextures>,
    ingredients: Query<(&Ingredient, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = cameras.get_single().unwrap();
    let window = windows.get_single().unwrap();
    for event in click_reader.iter() {
        let ingredient = ingredients
            .get_component::<Ingredient>(event.0)
            .unwrap_or(&Ingredient::Mushroom);
        let scale = ingredients
            .get_component::<GlobalTransform>(event.0)
            .unwrap()
            .compute_transform()
            .scale;
        let mouse = screen_to_world(
            Vec2::new(window.width(), window.height()),
            window.cursor_position().unwrap(),
            camera,
            camera_transform,
        );
        let id = spawn_ingredient(
            &mut commands,
            &textures,
            *ingredient,
            Transform::from_xyz(mouse.x, mouse.y, 0.1).with_scale(scale),
        );
        *drag_state.as_mut() = IngredientDragState::Dragging {
            entity: id,
            position: mouse,
        }
    }
}

fn drop_ingredient(
    mut commands: Commands,
    ingredients: Query<&Ingredient>,
    mouse_button: Res<Input<MouseButton>>,
    mut drag_state: ResMut<IngredientDragState>,
) {
    let mut drag_state = drag_state.as_mut();
    if mouse_button.just_released(MouseButton::Left) {
        match drag_state {
            IngredientDragState::Dragging { entity, position } => {
                commands
                    .get_entity(entity.clone())
                    .unwrap()
                    .insert(Gravity::default());
                *drag_state = IngredientDragState::None;
            }
            IngredientDragState::None => {}
        }
    }
}

// Just copied this might need work
fn screen_to_world(
    window_size: Vec2,
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    world_pos.truncate()
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

fn drag_item(
    drag_state: Res<IngredientDragState>,
    mut transforms: Query<&mut Transform>,
    window: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    match drag_state.into_inner() {
        IngredientDragState::Dragging { entity, position } => {
            let window = window.get_single().unwrap();
            let (camera, camera_transform) = cameras.get_single().unwrap();
            let mouse = screen_to_world(
                Vec2::new(window.width(), window.height()),
                window.cursor_position().unwrap_or(Vec2::new(0.0, 0.0)),
                camera,
                camera_transform,
            );
            if let Ok(mut transform) = transforms.get_component_mut::<Transform>(entity.clone()) {
                transform.translation.x = mouse.x;
                transform.translation.y = mouse.y;
            }
        }
        IngredientDragState::None => {}
    }
}

fn put_ingredients_on_shelf(
    mut commands: Commands,
    textures: Res<IngredientTextures>,
    shelf: Res<Shelf>,
) {
    let size = 24.0;
    let bx = ClickBox {
        topleft: Vec2 { x: -size, y: size },
        bottomright: Vec2 { x: size, y: -size },
    };

    // Mushroom
    let mushroom = spawn_ingredient(
        &mut commands,
        &textures,
        Ingredient::Mushroom,
        Transform::from_xyz(-240.0, -50.0, 0.1).with_scale(Vec3::splat(0.4)),
    );
    commands.get_entity(mushroom).unwrap().insert(bx);
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
    commands.get_entity(deerpiss).unwrap().insert(bx);
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
    commands.get_entity(toenails).unwrap().insert(bx);
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
    commands.get_entity(rabbitpoo).unwrap().insert(bx);
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
    commands.get_entity(fishhead).unwrap().insert(bx);
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
    commands.get_entity(froglegs).unwrap().insert(bx);
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
    commands.get_entity(corpseflower).unwrap().insert(bx);
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
    commands.get_entity(birtchbark).unwrap().insert(bx);
    commands.add(AddChild {
        parent: shelf.entity,
        child: birtchbark,
    });
}
