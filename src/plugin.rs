use bevy::prelude::*;

use crate::{ApngLoader, components::ApngAsset};

pub struct ApngPlugin;

impl Plugin for ApngPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ApngAsset>()
            .init_asset_loader::<ApngLoader>()
            .add_systems(
                Update,
                (
                    crate::systems::initialize_gifs,
                    crate::systems::animate_apngs,
                    crate::systems::despawn_apngs,
                ),
            );
    }
}
