use bevy::prelude::*;

use crate::tilemap_syncing;

#[derive(Default)]
pub struct TilesetIntePlugin;

pub static NEWCHUNKSTAGE: &str = "new_chunk_stage";

impl Plugin for TilesetIntePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<crate::tilemap_syncing::UpdateChunkEvent>()
            .add_event::<crate::tilemap_syncing::FullUpdateChunkEvent>()
            .add_stage_before(
                CoreStage::PostUpdate,
                NEWCHUNKSTAGE,
                SystemStage::single_threaded(),
            )
            .add_system_to_stage(NEWCHUNKSTAGE, tilemap_syncing::sync_full_chunk)
            .add_system_to_stage(CoreStage::PostUpdate, tilemap_syncing::sync_chunks);
    }
}
