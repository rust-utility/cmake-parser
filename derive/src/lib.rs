use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;

#[proc_macro_derive(CMake, attributes(cmake))]
#[proc_macro_error]
pub fn cmake_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let cmake_parse_path = quote! { ::cmake_parse:: };

    impl_cmake(&ast, cmake_parse_path)
}

#[proc_macro_derive(CMakeDirect, attributes(cmake))]
#[proc_macro_error]
pub fn cmake_direct_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let cmake_parse_path = quote! {};

    impl_cmake(&ast, cmake_parse_path)
}

fn impl_cmake(ast: &syn::DeriveInput, crate_path: proc_macro2::TokenStream) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let type_params = generics.type_params();
    let (_, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let data = &ast.data;

    let gen = match data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                quote! {
                    #[automatically_derived]
                    impl <'t #(, #type_params)*> #crate_path CMakeCommand<'t> for #name #ty_generics #where_clause {

                        fn parse<'tv>(
                            mut tokens: &'tv [Token<'t>],
                        ) -> Result<(Self, &'tv [Token<'t>]), #crate_path CommandParseError> {
                            todo!();
                        }

                    }
                }
            }
            syn::Fields::Unnamed(_) => todo!(),
            syn::Fields::Unit => todo!(),
        },
        syn::Data::Enum(_) => {
            let ass = proc_macro2::Literal::byte_string(b"ass");
            let test_field = quote! { #ass };
            quote! {
                #[automatically_derived]
                impl <'t #(, #type_params)*> #crate_path CMakeCommand<'t> for #name #ty_generics #where_clause {

                    fn parse<'tv>(
                        mut tokens: &'tv [Token<'t>],
                    ) -> Result<(Self, &'tv [Token<'t>]), #crate_path CommandParseError> {
                        let test_field = #test_field;
                        todo!();
                    }

                }
            }
        }
        syn::Data::Union(_) => todo!(),
    };
    gen.into()
}
