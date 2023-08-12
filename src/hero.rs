use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid, TypePath)]
#[uuid = "819647e5-e57c-49fc-adba-b0bbd5717ac4"]
pub struct HeroAsset {
    name: String,
}

pub struct HeroAssetLoader;

impl AssetLoader for HeroAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let deserialized = ron::de::from_bytes::<HeroAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(deserialized));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["hero.ron"]
    }
}
