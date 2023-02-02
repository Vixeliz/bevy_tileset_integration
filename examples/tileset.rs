use bevy::prelude::*;
use bevy_ecs_tilemap::{
    prelude::{fill_tilemap, TilemapId, TilemapTexture, TilemapTileSize, TilemapType},
    tiles::{AnimatedTile, TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};
use bevy_tileset::prelude::*;

use bevy_tileset_integration::prelude::*;

#[derive(Resource, Default)]
struct MyTileset {
    handle: Option<Handle<Tileset>>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(TilesetPlugin::default())
        .init_resource::<MyTileset>()
        .add_startup_system(load_tileset)
        .add_system(test_chunk)
        .run();
}

const VECTOR_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE as u32,
    y: CHUNK_SIZE as u32,
};

// This is all temporary I like to get a working prototype before fleshing out and cleaning up so I understand
// The libs and such
/// Temporary for me while debuging stuff.
fn test_chunk(
    mut commands: Commands,
    tilesets: Tilesets,
    my_tileset: Res<MyTileset>,
    mut has_ran: Local<bool>,
) {
    if my_tileset.handle.is_none() || *has_ran || !tilesets.contains_name("My Awesome Tileset") {
        return;
    }

    let handle = my_tileset.handle.as_ref().unwrap();
    if let Some(_) = tilesets.get(handle) {
        println!("Got tileset by handle! ({:?})", my_tileset.handle);
    }
    if let Some(tileset) = tilesets.get_by_id(&0) {
        println!("Got tileset by ID! ({})", tileset.id());
    }
    if let Some(tileset) = tilesets.get_by_name("My Awesome Tileset") {
        println!("Got tileset by name! ({})", tileset.name());
        println!("{:#?}", tileset);

        // === Generate Singular Chunk === //
        let mut chunk = Chunk::new(IVec2::new(0, 0), 0.0, Some("Dirt".to_string()));
        chunk.set_tile(UVec2::new(10, 10), "Wall".to_string());
        chunk.set_tile(UVec2::new(10, 9), "Wall".to_string());
        chunk.set_tile(UVec2::new(10, 8), "Glass".to_string());
        println!("{:?}", chunk);
        println!("{:?}", chunk.get_tile_id(UVec2::new(10, 10)));
        // === Bevy_Tileset Stuff === //
        commands.spawn(Camera2dBundle::default());

        // === Bevy_Ecs_Tilemap Stuff === //
        //Proof of concept for rendering
        let tilemap_entity = commands.spawn_empty().id();
        let mut tile_storage = TileStorage::empty(VECTOR_CHUNK_SIZE.into());
        let tile_size = TilemapTileSize {
            x: tileset.tile_size().x,
            y: tileset.tile_size().y,
        };
        // Spawn the elements of the tilemap.
        for x in 0..VECTOR_CHUNK_SIZE.x {
            for y in 0..VECTOR_CHUNK_SIZE.y {
                if let Some((ref tile_idx, ..)) = tileset.select_tile(
                    chunk
                        .get_tile_name(chunk.get_tile_id(UVec2::new(x, y)))
                        .as_str(),
                ) {
                    let tile_pos = TilePos { x, y };
                    let tile_entity = match tile_idx {
                        TileIndex::Standard(index) => commands
                            .spawn(TileBundle {
                                texture_index: TileTextureIndex((*index) as u32),
                                position: tile_pos,
                                tilemap_id: TilemapId(tilemap_entity),
                                ..Default::default()
                            })
                            .id(),
                        TileIndex::Animated(start, end, speed) => commands
                            .spawn(TileBundle {
                                texture_index: TileTextureIndex(0),
                                position: tile_pos,
                                tilemap_id: TilemapId(tilemap_entity),
                                ..Default::default()
                            })
                            .insert(AnimatedTile {
                                start: *start as u32,
                                end: *end as u32,
                                speed: *speed,
                            })
                            .id(),
                    };
                    commands.entity(tilemap_entity).add_child(tile_entity);
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        }
        let atlas = tileset.atlas();
        let texture = tileset.texture().clone();
        let transform = Transform::from_translation(Vec3::new(
            chunk.pos.x as f32 * VECTOR_CHUNK_SIZE.x as f32 * tile_size.x
                - (VECTOR_CHUNK_SIZE.x as f32 / 2.0 * tile_size.x),
            chunk.pos.y as f32 * VECTOR_CHUNK_SIZE.y as f32 * tile_size.y
                - (VECTOR_CHUNK_SIZE.y as f32 / 2.0 * tile_size.y),
            0.0,
        ));
        commands.entity(tilemap_entity).insert(TilemapBundle {
            grid_size: tile_size.into(),
            size: VECTOR_CHUNK_SIZE.into(),
            storage: tile_storage,
            tile_size: tile_size,
            transform,
            map_type: TilemapType::default(),
            texture: TilemapTexture::Single(texture),
            ..Default::default()
        });
        *has_ran = true;
    }
}

/// Starts the tileset loading process
fn load_tileset(mut my_tileset: ResMut<MyTileset>, asset_server: Res<AssetServer>) {
    my_tileset.handle = Some(asset_server.load("tilesets/my_tileset.ron"));
}
