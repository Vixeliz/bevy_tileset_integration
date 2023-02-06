// The way we handle bevy_ecs_tilemap is more of a way to just render tiles. So whenever the tiles get updated in our storage system.
// We just update the data for bevy_ecs_tilemap

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::*;
use tile_storage::CHUNK_SIZE;

use crate::{prelude::Chunk, tile_storage};

pub struct UpdateChunkEvent {
    pub tile_pos: TilePos,
    pub tileset_name: String,
    pub tile_type: String,
    pub entity: Entity,
}

pub struct FullUpdateChunkEvent {
    pub entity: Entity,
    pub tileset_name: String,
}

pub fn sync_full_chunk(
    mut commands: Commands,
    mut event: EventReader<FullUpdateChunkEvent>,
    mut tilemap_query: Query<(&mut Chunk, Entity, &mut TileStorage), With<Chunk>>,
    tilesets: Tilesets,
) {
    for evt in event.iter() {
        if let Some(tileset) = tilesets.get_by_name(evt.tileset_name.as_str()) {
            if let Ok((chunk, tilemap_id, mut tile_store)) = tilemap_query.get_mut(evt.entity) {
                for x in 0..CHUNK_SIZE {
                    for y in 0..CHUNK_SIZE {
                        let tile_pos = TilePos {
                            x: x as u32,
                            y: y as u32,
                        };
                        if let Some((ref tile_idx, ..)) = tileset.select_tile(
                            chunk
                                .get_tile_name(
                                    chunk.get_tile_id(UVec2::new(tile_pos.x, tile_pos.y)),
                                )
                                .as_str(),
                        ) {
                            let tile_entity = match tile_idx {
                                TileIndex::Standard(index) => commands
                                    .spawn(TileBundle {
                                        texture_index: TileTextureIndex((*index) as u32),
                                        position: tile_pos,
                                        tilemap_id: TilemapId(tilemap_id),
                                        ..Default::default()
                                    })
                                    .id(),
                                TileIndex::Animated(start, end, speed) => commands
                                    .spawn(TileBundle {
                                        texture_index: TileTextureIndex(0),
                                        position: tile_pos,
                                        tilemap_id: TilemapId(tilemap_id),
                                        ..Default::default()
                                    })
                                    .insert(AnimatedTile {
                                        start: *start as u32,
                                        end: *end as u32,
                                        speed: *speed,
                                    })
                                    .id(),
                            };
                            // commands.entity(tilemap_entity).add_child(tile_entity);
                            tile_store.set(&tile_pos, tile_entity);
                        }
                    }
                }
            }
        }
    }
}

pub fn sync_chunks(
    mut commands: Commands,
    mut event: EventReader<UpdateChunkEvent>,
    tilesets: Tilesets,
    mut tilemap_query: Query<(&mut Chunk, Entity, &mut TileStorage), With<Chunk>>,
) {
    for evt in event.iter() {
        if let Some(tileset) = tilesets.get_by_name(evt.tileset_name.as_str()) {
            if let Ok((mut chunk, tilemap_id, mut tile_store)) = tilemap_query.get_mut(evt.entity) {
                if let Some((ref tile_idx, ..)) = tileset.select_tile(evt.tile_type.as_str()) {
                    chunk.set_tile(evt.tile_pos.into(), evt.tile_type.to_owned());
                    let tile_entity = match tile_idx {
                        TileIndex::Standard(index) => commands
                            .spawn(TileBundle {
                                texture_index: TileTextureIndex((*index) as u32),
                                position: evt.tile_pos,
                                tilemap_id: TilemapId(tilemap_id),
                                ..Default::default()
                            })
                            .id(),
                        TileIndex::Animated(start, end, speed) => commands
                            .spawn(TileBundle {
                                texture_index: TileTextureIndex(0),
                                position: evt.tile_pos,
                                tilemap_id: TilemapId(tilemap_id),
                                ..Default::default()
                            })
                            .insert(AnimatedTile {
                                start: *start as u32,
                                end: *end as u32,
                                speed: *speed,
                            })
                            .id(),
                    };
                    // commands.entity(tilemap_entity).add_child(tile_entity);
                    tile_store.set(&evt.tile_pos, tile_entity);
                }
            }
        }
    }
}
