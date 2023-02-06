use std::collections::HashMap;

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_tileset::prelude::Tilesets;

use crate::prelude::{Chunk, ChunkBundle, FullUpdateChunkEvent};

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
    chunk_query: Query<'w, 's, &'static Chunk>,
    tilesets: Tilesets<'w, 's>,
    events: EventWriter<'w, 's, FullUpdateChunkEvent>,
}

impl<'w, 's> ChunkManager<'w, 's> {
    pub fn get_chunk(&self, pos: IVec2) {}
    pub fn add_new_chunk(&mut self, pos: IVec2, tileset_name: String) {
        if let Some(tileset) = self.tilesets.get_by_name(tileset_name.as_str()) {
            let chunk_entity = self.commands.spawn(ChunkBundle::new(&tileset, pos)).id();
            self.chunk_storage.chunks.insert(pos, chunk_entity);
            self.events.send(FullUpdateChunkEvent {
                entity: chunk_entity,
                tileset_name,
            });
        }
    }
    pub fn remove_chunk(&mut self, pos: IVec2) {
        self.commands
            .entity(*self.chunk_storage.chunks.get(&pos).unwrap());
        self.chunk_storage.chunks.remove(&pos);
        // self.events.send(FullUpdateChunkEvent {
        //     entity: chunk_entity,
        //     tileset_name,
        // });
    }
}
