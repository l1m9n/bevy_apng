use bevy::prelude::*;

use crate::prelude::ApngAsset;

#[derive(Message)]
pub(crate) struct ApngDespawnMessage(pub Handle<ApngAsset>);

// #[derive(Message)]
// pub struct ApngInitMessage(pub Handle<ApngAsset>);
