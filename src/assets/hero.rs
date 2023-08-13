use bevy::reflect::{TypePath, TypeUuid};
use serde::Deserialize;

use crate::generate_assets;

use asset_loader_derive::AssetLoader;

#[derive(Debug, Deserialize, TypeUuid, TypePath, AssetLoader)]
#[uuid = "819647e5-e57c-49fc-adba-b0bbd5717ac4"]
pub struct HeroAsset {
    name: String,
}
