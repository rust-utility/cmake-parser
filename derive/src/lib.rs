use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};

#[proc_macro_derive(CMake, attributes(cmake))]
#[proc_macro_error]
pub fn cmake_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_cmake(&ast)
}

fn impl_cmake(ast: &syn::DeriveInput) -> TokenStream {
    todo!()
}
