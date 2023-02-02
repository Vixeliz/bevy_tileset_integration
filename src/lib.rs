// If we use a custom storage method for tile types it would allow for easier getting of tiles at the cost of code complexity

mod placer;
mod plugin;
mod tile_storage;

pub mod prelude {
    pub use super::plugin::TilesetIntePlugin;
    pub use super::tile_storage::{Chunk, FixedTilemap, CHUNK_SIZE};
}
