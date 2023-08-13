extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Meta};

#[proc_macro_derive(AssetLoader, attributes(extensions))]
pub fn asset_loader_derive(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    // let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // ast.attrs.iter().for_each(|attr| {
    //     dbg!(attr);
    // });

    // impl_asset_loader(&ast)

    input.into()
}

fn impl_asset_loader(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let loader_name = Ident::new(
        &format!("{}Loader", name),
        proc_macro::Span::call_site().into(),
    );

    let gen = quote! {
        pub struct #loader_name;

            impl bevy::asset::AssetLoader for #loader_name {
                fn load<'a>(
                    &'a self,
                    bytes: &'a [u8],
                    load_context: &'a mut bevy::asset::LoadContext,
                ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
                    Box::pin(async move {
                        let deserialized = ron::de::from_bytes::<#name>(bytes)?;
                        load_context.set_default_asset(bevy::asset::LoadedAsset::new(deserialized));
                        Ok(())
                    })
                }

                fn extensions(&self) -> &[&str] {
                    &["test.ron"]
                }
            }
    };

    gen.into()
}
