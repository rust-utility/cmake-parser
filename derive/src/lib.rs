#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, quote_spanned};
use syn::DataEnum;

/// A derive macros for parsing CMake tokens to Rust structures and enums.
///
/// Requires dependency to `cmake-parser` crate.
#[proc_macro_derive(CMake, attributes(cmake))]
#[proc_macro_error]
pub fn cmake_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let cmake_parse_path = quote! { ::cmake_parser:: };

    impl_cmake(&ast, cmake_parse_path)
}

/// A derive macros for parsing CMake tokens to Rust structures and enums.
///
/// Requires all top level structure to be reimported from `cmake-parser`
/// in scope of derive (intended for internal usage in `cmake-parser` itself).
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
                let fields: Vec<_> = fields_named
                    .named
                    .iter()
                    .filter_map(|f| {
                        f.ident
                            .as_ref()
                            .map(|ident| (ident, cmake_attribute(&f.attrs)))
                    })
                    .map(|(ident, cmake_attr)| {
                        let id = ident.to_string();
                        let cmake_keyword = cmake_attr
                            .and_then(|a| a.rename)
                            .unwrap_or_else(|| id.to_uppercase());
                        let lit_cmake_keyword_str = proc_macro2::Literal::string(&cmake_keyword);
                        let lit_cmake_keyword_bstr =
                            proc_macro2::Literal::byte_string(cmake_keyword.as_bytes());
                        (ident, id, lit_cmake_keyword_str, lit_cmake_keyword_bstr)
                    })
                    .collect();

                let variables = fields.iter().map(|(ident, _, _, lit_cmake_keyword_bstr)| {
                    quote_spanned! { ident.span() => let mut #ident = #crate_path CMakeCommand::init(#lit_cmake_keyword_bstr, &mut keywords) }
                });
                let matches = fields.iter().map(|(ident, _, _, lit_cmake_keyword_bstr)| {
                    quote_spanned! { ident.span() => if #crate_path CMakeCommand::update(&mut #ident, #lit_cmake_keyword_bstr, decl.option(), decl.args())? { continue; } }
                });

                let struct_fields = fields.iter().map(|(ident, _, lit_cmake_keyword, _)| {
                    quote_spanned! { ident.span() => #ident: #ident.ok_or_else(|| #crate_path CommandParseError::MissingToken(#lit_cmake_keyword.to_string()))? }
                });

                quote! {
                    #[automatically_derived]
                    impl <'t #(, #type_params)*> #crate_path CMakeCommand<'t> for #name #ty_generics #where_clause {

                        fn parse<'tv>(
                            mut tokens: &'tv [#crate_path Token<'t>],
                        ) -> Result<(Self, &'tv [#crate_path Token<'t>]), #crate_path CommandParseError> {
                            let mut keywords = vec![];

                            #(#variables;)*

                            let declarations = #crate_path declarations_by_keywords(tokens, &keywords);

                            for decl in declarations {
                                #(#matches)*
                                return Err(#crate_path CommandParseError::UnknownOption(
                                    String::from_utf8_lossy(decl.option().as_bytes()).to_string(),
                                ));
                            }

                            Ok((
                                Self {
                                    #(#struct_fields,)*
                                },
                                &[],
                            ))
                        }
                    }
                }
            }
            syn::Fields::Unnamed(_) => {
                abort!(data_struct.fields, "unnamed fields are not supported")
            }
            syn::Fields::Unit => abort!(name, "unit fields are not supported"),
        },
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let fields: Vec<_> = variants
                .iter()
                .map(|f| (f.ident.clone(), cmake_attribute(&f.attrs)))
                .map(|(ident, cmake_attr)| {
                    let id = ident.to_string();
                    use inflections::Inflect;
                    let cmake_keyword = cmake_attr
                        .and_then(|a| a.rename)
                        .unwrap_or_else(|| id.to_constant_case());
                    let lit_cmake_keyword_str = proc_macro2::Literal::string(&cmake_keyword);
                    let lit_cmake_keyword_bstr =
                        proc_macro2::Literal::byte_string(cmake_keyword.as_bytes());
                    (ident, id, lit_cmake_keyword_str, lit_cmake_keyword_bstr)
                })
                .collect();

            let enum_keywords = fields.iter().map(|(ident, _, _, lit_cmake_keyword_bstr)| {
                quote_spanned! {ident.span() => #lit_cmake_keyword_bstr }
            });
            let matches = fields.iter().map(|(ident, _, _, lit_cmake_keyword_bstr)| {
                quote_spanned! {ident.span() => #lit_cmake_keyword_bstr => Self::#ident }
            });
            quote! {
                #[automatically_derived]
                impl <'t #(, #type_params)*> #crate_path CMakeCommand<'t> for #name #ty_generics #where_clause {

                    fn parse<'tv>(
                        mut tokens: &'tv [#crate_path Token<'t>],
                    ) -> Result<(Self, &'tv [#crate_path Token<'t>]), #crate_path CommandParseError> {
                        todo!();
                    }

                    fn init(_default_name: &'static [u8], keywords: &mut Vec<&'static [u8]>) -> Option<Self> {
                        let enum_keywords: &[&[u8]] = &[
                            #(#enum_keywords,)*
                        ];
                        keywords.extend(enum_keywords);
                        Self::default_value()
                    }

                    fn update(
                        command: &mut Option<Self>,
                        _expected: &'static [u8],
                        option: & #crate_path Token<'t>,
                        tokens: &[#crate_path Token<'t>],
                    ) -> Result<bool, #crate_path CommandParseError> {
                        let cmd = Some(match option.as_bytes() {
                            #(#matches,)*
                            _ => return Ok(false),
                        });

                        if !tokens.is_empty() {
                            return Err(#crate_path CommandParseError::Incomplete);
                        }

                        *command = cmd;

                        Ok(true)
                    }

                }
            }
        }
        syn::Data::Union(_) => abort!(name, "unions are not supported"),
    };
    gen.into()
}

struct CMakeAttribute {
    rename: Option<String>,
}

fn cmake_attribute(attrs: &[syn::Attribute]) -> Option<CMakeAttribute> {
    let attr = attrs.iter().find(|attr| attr.path().is_ident("cmake"))?;
    let mut rename = None;

    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("rename") {
            let expr: syn::LitStr = meta.value()?.parse()?;
            rename = Some(expr.value());
        }
        Ok(())
    })
    .unwrap();

    Some(CMakeAttribute { rename })
}
