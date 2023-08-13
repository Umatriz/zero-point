pub mod hero;

pub mod assets_macro {
    #[macro_export]
    macro_rules! generate_assets {
        (
            $asset_name: ident,
            $loader_ident:ident,
            $ext:expr
        ) => {
            pub struct $loader_ident;

            impl bevy::asset::AssetLoader for $loader_ident {
                fn load<'a>(
                    &'a self,
                    bytes: &'a [u8],
                    load_context: &'a mut bevy::asset::LoadContext,
                ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
                    Box::pin(async move {
                        let deserialized = ron::de::from_bytes::<$asset_name>(bytes)?;
                        load_context.set_default_asset(bevy::asset::LoadedAsset::new(deserialized));
                        Ok(())
                    })
                }

                fn extensions(&self) -> &[&str] {
                    $ext
                }
            }
        };
    }
}
