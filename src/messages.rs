use bevy::prelude::*;

use crate::ApngAsset;

#[derive(Message)]
pub(crate) struct ApngDespawnMessage(pub Handle<ApngAsset>);
