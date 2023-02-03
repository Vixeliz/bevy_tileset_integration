use bevy::prelude::*;

use crate::tilemap_syncing;

#[derive(Default)]
pub struct TilesetIntePlugin;

impl Plugin for TilesetIntePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<crate::tilemap_syncing::UpdateChunkEvent>()
            .add_system(tilemap_syncing::sync_chunks);
    }
}
