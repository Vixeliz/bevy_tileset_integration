use std::collections::HashMap;

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_tileset::prelude::Tilesets;

use crate::prelude::{Chunk, ChunkBundle, FullUpdateChunkEvent, UpdateChunkEvent, CHUNK_SIZE};

// This is the easy to use public facing api handles chunk creation,
// Creating and managing tilemaps with ecs_tilemap etc.
#[derive(Resource)]
pub struct ChunkStorage {
    pub chunks: HashMap<i32, HashMap<IVec2, Entity>>,
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
    /// Returns chunk_pos and the tile position relative to that chunk. WARNING: will return 0, 0 if the world position doesn't correspond to a chunk on that layer
    pub fn chunk_pos_from_world(
        &self,
        pos: Vec2,
        layer: i32,
        tileset_name: String,
    ) -> (IVec2, UVec2) {
        if let Some(current_chunk) = self.chunk_storage.chunks.get(&layer) {
            if let Some(tileset) = self.tilesets.get_by_name(tileset_name.as_str()) {
                let mut world_pos = (pos + tileset.tile_size() / 2.0).as_ivec2();
                if pos.x < -5.0 {
                    world_pos.x -= CHUNK_SIZE as i32 * tileset.tile_size().x as i32;
                }
                if pos.y < -5.0 {
                    world_pos.y -= CHUNK_SIZE as i32 * tileset.tile_size().y as i32;
                }
                let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE as i32, CHUNK_SIZE as i32);
                let tile_size: IVec2 =
                    IVec2::new(tileset.tile_size().x as i32, tileset.tile_size().y as i32);
                let conv_pos = world_pos / (chunk_size * tile_size);
                if let Some(_) = current_chunk.get(&conv_pos) {
                    (
                        conv_pos,
                        UVec2::new(
                            (world_pos.x as f32 / tileset.tile_size().x)
                                .rem_euclid(chunk_size.x as f32) as u32,
                            (world_pos.y as f32 / tileset.tile_size().y)
                                .rem_euclid(chunk_size.y as f32) as u32,
                        ),
                    )
                } else {
                    // warn!("Couldn't convert position to a valid chunk");
                    (IVec2::new(0, 0), UVec2::new(0, 0))
                }
            } else {
                // warn!("Couldn't get tileset");
                (IVec2::new(0, 0), UVec2::new(0, 0))
            }
        } else {
            // warn!("Non existent layer");
            (IVec2::new(0, 0), UVec2::new(0, 0))
        }
    }
    pub fn add_chunk(&mut self, tileset_name: String, chunk: Chunk) {
        if let Some(tileset) = self.tilesets.get_by_name(tileset_name.as_str()) {
            let pos = chunk.pos;
            let layer = chunk.layer;
            let chunk_entity = self
                .commands
                .spawn(ChunkBundle::new_from_chunk(&tileset, chunk))
                .id();
            if let Some(current_chunk) = self.chunk_storage.chunks.get_mut(&layer) {
                current_chunk.insert(pos, chunk_entity);
                self.full_update_chunk_events.send(FullUpdateChunkEvent {
                    entity: chunk_entity,
                    tileset_name,
                });
            } else {
                self.chunk_storage.chunks.insert(layer, HashMap::new());
                let current_chunk = self.chunk_storage.chunks.get_mut(&layer).unwrap();
                current_chunk.insert(pos, chunk_entity);
                self.full_update_chunk_events.send(FullUpdateChunkEvent {
                    entity: chunk_entity,
                    tileset_name,
                });
            }
        }
    }
    pub fn add_new_chunk(&mut self, pos: IVec2, tileset_name: String, layer: i32) {
        if let Some(tileset) = self.tilesets.get_by_name(tileset_name.as_str()) {
            let chunk_entity = self
                .commands
                .spawn(ChunkBundle::new(&tileset, pos, layer))
                .id();
            if let Some(current_chunk) = self.chunk_storage.chunks.get_mut(&layer) {
                current_chunk.insert(pos, chunk_entity);
                self.full_update_chunk_events.send(FullUpdateChunkEvent {
                    entity: chunk_entity,
                    tileset_name,
                });
            } else {
                self.chunk_storage.chunks.insert(layer, HashMap::new());
                let current_chunk = self.chunk_storage.chunks.get_mut(&layer).unwrap();
                current_chunk.insert(pos, chunk_entity);
                self.full_update_chunk_events.send(FullUpdateChunkEvent {
                    entity: chunk_entity,
                    tileset_name,
                });
            }
        }
    }
    pub fn remove_chunk(&mut self, pos: IVec2, layer: i32) {
        if let Some(current_chunk) = self.chunk_storage.chunks.get_mut(&layer) {
            if let Some(chunk_entity) = current_chunk.get(&pos) {
                self.commands.entity(*chunk_entity).despawn_recursive();
                current_chunk.remove(&pos);
            }
        }
    }
    pub fn set_tile_in_chunk(
        &mut self,
        pos: UVec2,
        chunk_pos: IVec2,
        tileset_name: String,
        block_type: String,
        layer: i32,
    ) {
        if let Some(current_chunk) = self.chunk_storage.chunks.get(&layer) {
            if let Some(chunk_entity) = current_chunk.get(&chunk_pos) {
                self.update_chunk_events.send(UpdateChunkEvent {
                    tile_pos: TilePos { x: pos.x, y: pos.y },
                    tileset_name,
                    entity: *chunk_entity,
                    tile_type: block_type,
                });
            }
        }
    }
}
