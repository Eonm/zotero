extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ItemCommon)]
pub fn tagable_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_tagable_macro(&ast)
}

fn  impl_tagable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = match &ast.data {
        syn::Data::Enum(d) => {
            let variants = &d.variants;

            // implement tags fuction fo enum values for every variant
            let mut variant_impls_tags = Vec::new();
            for v in variants {
                let vid = &v.ident;
                variant_impls_tags.push(
                    quote!{
                        #name::#vid(x) => x.tags()
                    }
                )
            }

            // implement title fuction fo enum values for every variant
            let mut variant_impls_title = Vec::new();
            for v in variants {
                let vid = &v.ident;
                variant_impls_title.push(
                    quote!{
                        #name::#vid(x) => x.title()
                    }
                )
            }

            // implement key fuction fo enum values for every variant
            let mut variant_impls_key = Vec::new();
            for v in variants {
                let vid = &v.ident;
                variant_impls_key.push(
                    quote!{
                        #name::#vid(x) => x.key()
                    }
                )
            }

            quote!{
                impl ItemCommon for #name {
                    fn title(&self) -> &str {
                        match &self {
                            #(#variant_impls_title),*
                        }
                    }

                    fn key(&self) -> &str {
                        match &self {
                            #(#variant_impls_key),*
                        }
                    }

                    fn tags(&self) -> &Vec<Tag> {
                        match &self {
                            #(#variant_impls_tags),*
                        }
                    }
                }
            }
        }
        syn::Data::Struct(_d) => {
            quote! {
                impl ItemCommon for #name {
                    fn title(&self) -> &str {
                        &self.title
                    }

                    fn key(&self) -> &str {
                        &self.key
                    }

                    fn tags(&self) -> &Vec<Tag> {
                        &self.tags
                    }
                }
            }
        }
        _ => {
            panic!("#[derive(ItemCommon)] is only available for Enum and Struct Types!")
        }
    };
    gen.into()
}
