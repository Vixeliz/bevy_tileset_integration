use bevy::prelude::*;
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    tiles::TileStorage,
    TilemapBundle,
};
use bevy_tileset::prelude::Tileset;

use crate::prelude::{Chunk, CHUNK_SIZE};

/// A bevy bundle for building a new chunk containing the abstracted storage and bevy_ecs_tilemap
/// Components.
#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk: Chunk,
    #[bundle]
    pub tilemap_bundle: TilemapBundle,
}

// In the future we could possibly use systemparams instead with something that handles the chunks
// But rn i want to keep it relatively simple the code so it's easy to maintain and understand.
// TODO: pass in position
impl ChunkBundle {
    pub fn new(tileset: &Tileset) -> ChunkBundle {
        let chunk = Chunk::default();
        ChunkBundle {
            chunk,
            tilemap_bundle: TilemapBundle {
                grid_size: TilemapGridSize {
                    x: tileset.tile_size().x,
                    y: tileset.tile_size().y,
                },
                map_type: TilemapType::default(),
                size: TilemapSize {
                    x: CHUNK_SIZE as u32,
                    y: CHUNK_SIZE as u32,
                },
                storage: TileStorage::empty(TilemapSize {
                    x: CHUNK_SIZE as u32,
                    y: CHUNK_SIZE as u32,
                }),
                texture: TilemapTexture::Single(tileset.texture().clone()),
                tile_size: TilemapTileSize {
                    x: tileset.tile_size().x,
                    y: tileset.tile_size().y,
                },
                ..Default::default()
            },
        }
    }
}
