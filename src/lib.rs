// If we use a custom storage method for tile types it would allow for easier getting of tiles at the cost of code complexity

mod components;
mod placer;
mod plugin;
mod tile_storage;
mod tilemap_syncing;

pub mod prelude {
    pub use super::components::ChunkBundle;
    pub use super::plugin::TilesetIntePlugin;
    pub use super::tile_storage::{Chunk, FixedTilemap, CHUNK_SIZE};
    pub use super::tilemap_syncing::{FullUpdateChunkEvent, UpdateChunkEvent};
}
