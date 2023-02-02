// This is opinionated as we could just store tiles using bevy_ecs_tilemap. However I don't personally like how it uses a texture atlas id for storage.
// So instead we are using chunks that store the name of a tile. It will always use chunks internally but have an api that offers both chunk access and an
// option for a fixed tilemap. As of right now layers will be handled by just making another one of our tilemap storage options.
use std::collections::HashMap;

use bevy::prelude::{IVec2, UVec2, Vec2};

const CHUNK_SIZE: usize = 64;

// We are stealing how minecraft and some other engines store chunk blocks. Instead of storing everything as a string per tile(which we could probably get away with in 2d)
// we store a pallette which will map a tile string to a number then the tiles are stored as the numbers. This allows us to save on memory in most cases where there are same tiles

// Chunks are on the user to deal with in terms of deciding when to spawn or remove them. The only case that isn't true is when using tilemap where they will just always be there unless
// the user chooses to do anything with them.
#[derive(Debug)]
pub struct Chunk {
    pub palette: HashMap<String, u16>,
    pub tiles: [u16; CHUNK_SIZE * CHUNK_SIZE],
    pub pos: IVec2,
    pub layer: f32,
}

impl Chunk {
    /// Makes a new chunk filled with nothing.
    pub fn new(pos: IVec2, layer: f32) -> Chunk {
        let mut new_chunk = Chunk {
            palette: HashMap::new(),
            tiles: [0; CHUNK_SIZE * CHUNK_SIZE],
            pos,
            layer,
        };
        new_chunk.palette.insert("Air".to_string(), 0);
        new_chunk
    }
    // 0 is always air
    fn add_tile_to_chunk(&mut self, tile: String) {
        // This is kind of a stupid way to do this
        for i in 1..self.tiles.len() {
            self.palette.entry(tile).or_insert(i as u16);
            return;
        }
    }
    pub fn get_tile_id(&self, coords: UVec2) -> u16 {
        self.tiles[(coords.x + coords.y * CHUNK_SIZE as u32) as usize]
    }
    pub fn set_tile(&mut self, coords: UVec2, tile: String) {
        if self.palette.contains_key(&tile) {
            self.tiles[(coords.x + coords.y * CHUNK_SIZE as u32) as usize] =
                *self.palette.get(&tile).expect("No value");
        } else {
            self.add_tile_to_chunk(tile.clone());
            self.tiles[(coords.x + coords.y * CHUNK_SIZE as u32) as usize] =
                *self.palette.get(&tile).expect("No value");
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
                    .insert(chunk_pos_index, Chunk::new(IVec2::new(x, y), layer));
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
