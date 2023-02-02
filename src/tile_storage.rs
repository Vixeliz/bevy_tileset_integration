// This is opinionated as we could just store tiles using bevy_ecs_tilemap. However I don't personally like how it uses a texture atlas id for storage.
// So instead we are using chunks that store the name of a tile. It will always use chunks internally but have an api that offers both chunk access and an
// option for a fixed tilemap. As of right now layers will be handled by just making another one of our tilemap storage options.
use std::collections::HashMap;

use bevy::prelude::{Component, IVec2, UVec2, Vec2};
use bimap::BiMap;

pub const CHUNK_SIZE: usize = 64;

// We are stealing how minecraft and some other engines store chunk blocks. Instead of storing everything as a string per tile(which we could probably get away with in 2d)
// we store a pallette which will map a tile string to a number then the tiles are stored as the numbers. This allows us to save on memory in most cases where there are same tiles

// Chunks are on the user to deal with in terms of deciding when to spawn or remove them. The only case that isn't true is when using tilemap where they will just always be there unless
// the user chooses to do anything with them.
#[derive(Debug, Component)]
pub struct Chunk {
    pub palette: BiMap<String, u16>,
    pub tiles: [u16; CHUNK_SIZE * CHUNK_SIZE],
    pub pos: IVec2,
    pub layer: f32,
}

impl Chunk {
    /// Makes a new chunk filled with given tile or nothing.
    pub fn new(pos: IVec2, layer: f32, tile: Option<String>) -> Chunk {
        let mut new_chunk = Chunk {
            palette: BiMap::new(),
            tiles: [0; CHUNK_SIZE * CHUNK_SIZE],
            pos,
            layer,
        };
        match tile {
            Some(x) => {
                new_chunk.palette.insert(x, 1);
                new_chunk.tiles = [1; CHUNK_SIZE * CHUNK_SIZE];
                new_chunk.palette.insert("Air".to_string(), 0);
            }
            None => {
                new_chunk.palette.insert("Air".to_string(), 0);
            }
        }
        new_chunk
    }
    // 0 is always air
    fn add_tile_to_chunk(&mut self, tile: String) {
        // This is kind of a stupid way to do this
        for i in 1..self.tiles.len() {
            if self.palette.contains_left(&tile) {
                return;
            }
            if self.palette.contains_right(&(i as u16)) {
                continue;
            }
            self.palette.insert(tile, i as u16);
            return;
        }
    }
    pub fn get_tile_id(&self, coords: UVec2) -> u16 {
        self.tiles[(coords.x + coords.y * CHUNK_SIZE as u32) as usize]
    }
    pub fn get_tile_name(&self, tile_id: u16) -> String {
        for key in self.palette.left_values() {
            if let Some(value) = self.palette.get_by_left(key) {
                if *value == tile_id {
                    return key.clone();
                }
            }
        }
        "Air".to_string()
    }
    pub fn set_tile(&mut self, coords: UVec2, tile: String) {
        if self.palette.contains_left(&tile) {
            self.tiles[(coords.x + coords.y * CHUNK_SIZE as u32) as usize] =
                *self.palette.get_by_left(&tile).expect("No value");
        } else {
            self.add_tile_to_chunk(tile.clone());
            self.tiles[(coords.x + coords.y * CHUNK_SIZE as u32) as usize] =
                *self.palette.get_by_left(&tile).expect("No value");
        }
    }
}

// Here is the optional fixed size map
pub struct FixedTilemap {
    pub size: IVec2,
    pub chunks: HashMap<i32, Chunk>,
}

impl FixedTilemap {
    /// Create new tilemap with the given size in tiles
    pub fn new(&self, size: IVec2, layer: f32) -> FixedTilemap {
        let mut new_fixed_tilemap = FixedTilemap {
            size,
            chunks: HashMap::new(),
        };
        for x in 0..(size.x / CHUNK_SIZE as i32) {
            for y in 0..(size.y / CHUNK_SIZE as i32) {
                let chunk_pos_index = x + y * (size.x / CHUNK_SIZE as i32);
                new_fixed_tilemap
                    .chunks
                    .insert(chunk_pos_index, Chunk::new(IVec2::new(x, y), layer, None));
            }
        }
        new_fixed_tilemap
    }
    pub fn get_chunk_from_tile(&self, tile_pos: IVec2) -> Option<&Chunk> {
        let chunk_pos_index = ((tile_pos.x / CHUNK_SIZE as i32)
            + (tile_pos.y / CHUNK_SIZE as i32) * (self.size.x / CHUNK_SIZE as i32))
            as i32;
        self.chunks.get(&chunk_pos_index)
    }
}
