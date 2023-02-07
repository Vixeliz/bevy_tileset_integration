use bevy::{prelude::*, render::camera::RenderTarget};
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::*;

use bevy_tileset_integration::prelude::*;

use rand::Rng;

#[derive(Resource, Default)]
struct MyTileset {
    handle: Option<Handle<Tileset>>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(TilemapPlugin)
        .add_plugin(TilesetPlugin::default())
        .add_plugin(TilesetIntePlugin)
        .init_resource::<MyTileset>()
        .init_resource::<WorldMousePos>()
        .add_startup_system(load_tileset)
        .add_system(test_chunk)
        .add_system(move_camera)
        .add_system(my_cursor_system)
        .add_system(tile_set)
        .run();
}

#[derive(Component)]
pub struct CameraTag;

fn test_chunk(
    mut commands: Commands,
    tilesets: Tilesets,
    my_tileset: Res<MyTileset>,
    mut has_ran: Local<bool>,
    mut chunk_manager: ChunkManager,
) {
    if my_tileset.handle.is_none() || *has_ran || !tilesets.contains_name("My Awesome Tileset") {
        return;
    }

    if let Some(tileset) = tilesets.get_by_name("My Awesome Tileset") {
        let mut camera = Camera2dBundle::default();
        camera.projection.scale = 0.5;
        commands.spawn((camera, CameraTag));
        let chunk = Chunk::new(IVec2 { x: 0, y: 0 }, 1, Some("Grass".to_string()));
        chunk_manager.add_chunk("My Awesome Tileset".to_string(), chunk);
        chunk_manager.add_new_chunk(IVec2 { x: -1, y: 0 }, "My Awesome Tileset".to_string(), 1);
        chunk_manager.add_new_chunk(IVec2 { x: -1, y: -1 }, "My Awesome Tileset".to_string(), 1);
        chunk_manager.add_new_chunk(IVec2 { x: 0, y: -1 }, "My Awesome Tileset".to_string(), 1);
        *has_ran = true;
    }
}

// Starts the tileset loading process
fn load_tileset(mut my_tileset: ResMut<MyTileset>, asset_server: Res<AssetServer>) {
    my_tileset.handle = Some(asset_server.load("tilesets/my_tileset.ron"));
}

#[derive(Resource, Default)]
struct WorldMousePos(Vec2);

// Converts cursor screen pos to world pos. Not my code found from bevy cheatbook
fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<CameraTag>>,
    mut world_mouse: ResMut<WorldMousePos>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    if let Ok((camera, camera_transform)) = q_camera.get_single() {
        // get the window that the camera is displaying to (or the primary window)
        let wnd = if let RenderTarget::Window(id) = camera.target {
            wnds.get(id).unwrap()
        } else {
            wnds.get_primary().unwrap()
        };

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();
            world_mouse.0 = world_pos;
        }
    }
}

fn move_camera(
    mut q_camera: Query<&mut Transform, With<CameraTag>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut camera_transform) = q_camera.get_single_mut() {
        if keys.pressed(KeyCode::W) {
            camera_transform.translation.y += 500.0 * time.delta_seconds();
        }
        if keys.pressed(KeyCode::S) {
            camera_transform.translation.y -= 500.0 * time.delta_seconds();
        }
        if keys.pressed(KeyCode::D) {
            camera_transform.translation.x += 500.0 * time.delta_seconds();
        }
        if keys.pressed(KeyCode::A) {
            camera_transform.translation.x -= 500.0 * time.delta_seconds();
        }
    }
}

fn tile_set(
    mut chunk_manager: ChunkManager,
    chunk_query: Query<&Chunk>,
    world_mouse: Res<WorldMousePos>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (chunk_pos, tile_pos) =
            chunk_manager.chunk_pos_from_world(world_mouse.0, 1, "My Awesome Tileset".to_string());
        chunk_manager.set_tile_in_chunk(
            tile_pos,
            chunk_pos,
            "My Awesome Tileset".to_string(),
            "Dirt".to_string(),
            1,
        );
    }
    //= How you could delete a chunk =//
    //chunk_manager.remove_chunk(key, 0);
}
