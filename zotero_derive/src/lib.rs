extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Tagable)]
pub fn tagable_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_tagable_macro(&ast)
}

fn  impl_tagable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = match &ast.data {
        syn::Data::Enum(d) => {
            let variants = &d.variants;
            let mut variant_impls = Vec::new();
            for v in variants {
                let vid = &v.ident;
                variant_impls.push(
                    quote!{
                        #name::#vid(x) => x.tags()
                    }
                )
            }
            quote!{
                impl Tagable for #name {
                    fn tags(&self) -> &Vec<Tag> {
                        match &self {
                            #(#variant_impls),*
                        }
                    }
                }
            }
        }
        syn::Data::Struct(_d) => {
            quote! {
                impl Tagable for #name {
                    fn tags(&self) -> &Vec<Tag> {
                        &self.tags
                    }
                }
            }
        }
        _ => {
            panic!("#[derive(Tagable)] is only available for Enum and Struct Types!")
        }
    };
    gen.into()
}
