use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapBundle;

use crate::prelude::Chunk;

#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk: Chunk,
    #[bundle]
    pub tilemap_bundle: TilemapBundle,
}
