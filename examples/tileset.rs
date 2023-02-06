use bevy::prelude::*;
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
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(TilesetPlugin::default())
        .add_plugin(TilesetIntePlugin)
        .init_resource::<MyTileset>()
        .add_startup_system(load_tileset)
        .add_system(test_chunk)
        .add_system(random_tiles)
        .run();
}

// This is all temporary I like to get a working prototype before fleshing out and cleaning up so I understand
// The libs and such
/// Temporary for me while debuging stuff.
fn test_chunk(
    mut commands: Commands,
    tilesets: Tilesets,
    my_tileset: Res<MyTileset>,
    mut has_ran: Local<bool>,
    // mut events: EventWriter<FullUpdateChunkEvent>,
    mut chunk_manager: ChunkManager,
) {
    if my_tileset.handle.is_none() || *has_ran || !tilesets.contains_name("My Awesome Tileset") {
        return;
    }

    // let handle = my_tileset.handle.as_ref().unwrap();
    if let Some(tileset) = tilesets.get_by_name("My Awesome Tileset") {
        // === Generate Singular Chunk === //
        let mut chunk = Chunk::new(IVec2::new(0, 0), 0.0, Some("Dirt".to_string()));
        chunk.set_tile(UVec2::new(10, 10), "Wall".to_string());
        chunk.set_tile(UVec2::new(10, 9), "Wall".to_string());
        chunk.set_tile(UVec2::new(10, 8), "Glass".to_string());

        commands.spawn(Camera2dBundle::default());
        // Spawn the new chunk default chunk passing in the tileset from bevy_tileset
        // commands.spawn(ChunkBundle::new(&tileset, IVec2 { x: -1, y: 0 }));
        // // Create new chunk entity from existing chunk
        // let entity_id = commands
        //     .spawn(ChunkBundle::new_from_chunk(&tileset, chunk))
        //     .id();
        // events.send(FullUpdateChunkEvent {
        //     entity: entity_id,
        //     tileset_name: "My Awesome Tileset".to_string(),
        // });
        chunk_manager.add_new_chunk(IVec2 { x: 0, y: 0 }, "My Awesome Tileset".to_string());
        *has_ran = true;
    }
}

// Starts the tileset loading process
fn load_tileset(mut my_tileset: ResMut<MyTileset>, asset_server: Res<AssetServer>) {
    my_tileset.handle = Some(asset_server.load("tilesets/my_tileset.ron"));
}

fn random_tiles(mut chunk_manager: ChunkManager) {
    // Just randomly set tiles in the given chunk. Would have to loop if you were
    // spawning multiples
    let block_type = match rand::thread_rng().gen_range(0..4) {
        0 => "Grass".to_string(),
        1 => "Dirt".to_string(),
        2 => "Glass".to_string(),
        _ => "Wall".to_string(),
    };
    let tile_pos = UVec2::new(
        rand::thread_rng().gen_range(0..CHUNK_SIZE) as u32,
        rand::thread_rng().gen_range(0..CHUNK_SIZE) as u32,
    );
    chunk_manager.set_tile_in_chunk(
        tile_pos,
        IVec2 { x: 0, y: 0 },
        "My Awesome Tileset".to_string(),
        block_type,
    );
    if rand::thread_rng().gen_range(0..1000) == 0 {
        chunk_manager.remove_chunk(IVec2 { x: 0, y: 0 });
    }
}
