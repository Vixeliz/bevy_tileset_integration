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
impl ChunkBundle {
    pub fn new(tileset: &Tileset, pos: IVec2) -> ChunkBundle {
        let mut chunk = Chunk::default();
        chunk.pos = pos;
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
                transform: Transform::from_translation(Vec3::new(
                    pos.x as f32 * CHUNK_SIZE as f32 * tileset.tile_size().x,
                    pos.y as f32 * CHUNK_SIZE as f32 * tileset.tile_size().y,
                    0.0,
                )),
                ..Default::default()
            },
        }
    }
    pub fn new_from_chunk(tileset: &Tileset, chunk: Chunk) -> ChunkBundle {
        let (x, y) = (chunk.pos.x, chunk.pos.y);
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
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * CHUNK_SIZE as f32 * tileset.tile_size().x,
                    y as f32 * CHUNK_SIZE as f32 * tileset.tile_size().y,
                    0.0,
                )),
                ..Default::default()
            },
        }
    }
}
