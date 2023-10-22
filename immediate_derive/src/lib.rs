use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Immediate)]
pub fn immediate_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_immediate(&ast)
}

fn impl_immediate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl From<u32> for #name {
            fn from(value: u32) -> Self {
                Self {
                    imm: value
                }
            }
        }

        impl Into<u32> for #name {
            fn into(self) -> u32 {
                self.imm
            }
        }

        impl PartialEq<u32> for #name {
            fn eq(&self, other: &u32) -> bool {
                other.eq(&self.imm)
            }
        }
    };
    gen.into()
}
