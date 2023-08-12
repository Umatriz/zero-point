use bevy::prelude::*;
use hero::{HeroAsset, HeroAssetLoader};

pub mod hero;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_asset::<HeroAsset>()
        .add_asset_loader(HeroAssetLoader)
        .run()
}

fn setup(asset_server: Res<AssetServer>) {
    let asset_handle: Handle<HeroAsset> = asset_server.load("test.hero.ron");
}
