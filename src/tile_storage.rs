// This is opinionated as we could just store tiles using bevy_ecs_tilemap. However I don't personally like how it uses a texture atlas id for storage.
// So instead we are using chunks that store the name of a tile. It will always use chunks internally but have an api that offers both chunk access and an
// option for a fixed tilemap.
use std::collections::HashMap;

use bevy::prelude::Vec2;

const CHUNK_SIZE: usize = 64;

// We are stealing how minecraft and some other engines store chunk blocks. Instead of storing everything as a string per tile(which we could probably get away with in 2d)
// we store a pallette which will map a tile string to a number then the tiles are stored as the numbers. This allows us to save on memory in most cases where there are same tiles

pub struct Chunk {
    pub palette: HashMap<String, u16>,
    pub tiles: [u16; CHUNK_SIZE * CHUNK_SIZE],
}

impl Chunk {
    /// Makes a new chunk filled with nothing.
    pub fn new() -> Chunk {
        let mut new_chunk = Chunk {
            palette: HashMap::new(),
            tiles: [0; CHUNK_SIZE * CHUNK_SIZE],
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
    pub fn get_tile_id(&self, coords: Vec2) -> u16 {
        self.tiles[coords.x as usize + coords.y as usize * CHUNK_SIZE]
    }
    pub fn set_tile(&mut self, coords: Vec2, tile: String) {
        if self.palette.contains_key(&tile) {
            self.tiles[coords.x as usize + coords.y as usize * CHUNK_SIZE] =
                *self.palette.get(&tile).expect("No value");
        } else {
            self.add_tile_to_chunk(tile.clone());
            self.tiles[coords.x as usize + coords.y as usize * CHUNK_SIZE] =
                *self.palette.get(&tile).expect("No value");
        }
    }
}
