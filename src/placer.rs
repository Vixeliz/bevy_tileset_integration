use std::collections::HashMap;

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_tileset::prelude::Tilesets;

use crate::prelude::{Chunk, ChunkBundle, FullUpdateChunkEvent, UpdateChunkEvent};

// This is the easy to use public facing api handles chunk creation,
// Creating and managing tilemaps with ecs_tilemap etc.
#[derive(Resource)]
pub struct ChunkStorage {
    pub chunks: HashMap<IVec2, Entity>,
}
#[derive(SystemParam)]
pub struct ChunkManager<'w, 's> {
    commands: Commands<'w, 's>,
    chunk_storage: ResMut<'w, ChunkStorage>,
    tilesets: Tilesets<'w, 's>,
    full_update_chunk_events: EventWriter<'w, 's, FullUpdateChunkEvent>,
    update_chunk_events: EventWriter<'w, 's, UpdateChunkEvent>,
}

impl<'w, 's> ChunkManager<'w, 's> {
    pub fn get_chunk(&self, pos: IVec2) {}
    pub fn add_new_chunk(&mut self, pos: IVec2, tileset_name: String) {
        if let Some(tileset) = self.tilesets.get_by_name(tileset_name.as_str()) {
            let chunk_entity = self.commands.spawn(ChunkBundle::new(&tileset, pos)).id();
            self.chunk_storage.chunks.insert(pos, chunk_entity);
            self.full_update_chunk_events.send(FullUpdateChunkEvent {
                entity: chunk_entity,
                tileset_name,
            });
        }
    }
    pub fn remove_chunk(&mut self, pos: IVec2) {
        if let Some(chunk_entity) = self.chunk_storage.chunks.get(&pos) {
            self.commands.entity(*chunk_entity).despawn_recursive();
            self.chunk_storage.chunks.remove(&pos);
        }
    }
    pub fn set_tile_in_chunk(
        &mut self,
        pos: UVec2,
        chunk_pos: IVec2,
        tileset_name: String,
        block_type: String,
    ) {
        if let Some(chunk_entity) = self.chunk_storage.chunks.get(&chunk_pos) {
            self.update_chunk_events.send(UpdateChunkEvent {
                tile_pos: TilePos { x: pos.x, y: pos.y },
                tileset_name,
                entity: *chunk_entity,
                tile_type: block_type,
            });
        }
    }
}
