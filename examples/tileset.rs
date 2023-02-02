use bevy::prelude::*;
use bevy_tileset::prelude::*;

use bevy_tileset_integration::prelude::*;

#[derive(Resource, Default)]
struct MyTileset {
    handle: Option<Handle<Tileset>>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilesetPlugin::default())
        .init_resource::<MyTileset>()
        .add_startup_system(load_tileset)
        .add_startup_system(test_chunk)
        // .add_system(show_tileset)
        .run();
}

/// Temporary for me while debuging stuff.
fn test_chunk() {
    let mut chunk = Chunk::new(UVec2::new(0, 0), 0.0);
    chunk.set_tile(UVec2::new(10, 10), "Grass".to_string());
    println!("{:?}", chunk);
    println!("{:?}", chunk.get_tile_id(UVec2::new(10, 10)));
}

/// Starts the tileset loading process
fn load_tileset(mut my_tileset: ResMut<MyTileset>, asset_server: Res<AssetServer>) {
    my_tileset.handle = Some(asset_server.load("tilesets/my_tileset.ron"));
}

/// Shows the tileset
///
/// This uses the `Tilesets` system parameter. Internally it gets the `Res<Assets<Tileset>>`, but also provides
/// additional niceties (specifically fetching a tileset by name or ID).
fn show_tileset(
    tilesets: Tilesets,
    mut commands: Commands,
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

        // === Display Tileset === //
        let atlas = tileset.atlas();
        let texture = tileset.texture().clone();
        commands.spawn(Camera2dBundle::default());
        commands.spawn(SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });

        // === Display Tile === //
        if let Some((ref tile_index, ..)) = tileset.select_tile("Wall") {
            match tile_index {
                TileIndex::Standard(index) => {
                    // Do something standard
                    commands.spawn(SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(08.0, -48.0, 0.0),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(*index),
                        texture_atlas: atlas.clone(),
                        ..Default::default()
                    });
                }
                TileIndex::Animated(start, end, speed) => {
                    // Do something  ✨ animated ✨
                }
            }
        }

        *has_ran = true;
    }
}
