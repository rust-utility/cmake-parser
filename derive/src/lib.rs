#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, quote_spanned};
use syn::{
    punctuated::Punctuated, DataEnum, DeriveInput, Expr, ExprLit, Lit, Meta, MetaNameValue, Token,
};

/// A derive macros for parsing CMake tokens to Rust structures and enums.
///
/// Requires dependency to `cmake-parser` crate.
#[proc_macro_derive(CMake, attributes(cmake))]
#[proc_macro_error]
pub fn cmake_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let cmake_attr = cmake_attribute(&ast.attrs).unwrap_or_default();
    let cmake_parse_path = if let Some(crate_path) = cmake_attr.pkg.as_ref() {
        quote! { #crate_path }
    } else {
        quote! { ::cmake_parser }
    };

    let cmake_impl = CMakeImpl::new(ast, cmake_parse_path);
    let trait_cmake_parse = if cmake_attr.positional {
        cmake_impl.trait_cmake_parse_positional()
    } else {
        cmake_impl.trait_cmake_parse_regular(cmake_attr)
    };

    let trait_cmake_positional = cmake_impl.trait_cmake_positional_regular();

    quote! {
        #trait_cmake_parse
        #trait_cmake_positional
    }
    .into()
}

fn enum_field_matches(
    variants: &[CMakeEnum],
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    variants.iter().map(
        |CMakeEnum {
             option:
                 CMakeOption {
                     ident,
                     lit_bstr,
                     attr: CMakeAttribute {
                        transparent, ..
                     },
                     ..
                 },
             unnamed,
         }| {
            let tokens = if *transparent {
                quote! { rest }
            } else {
                quote! { tokens }
            };
            if *unnamed {
                quote_spanned! { ident.span() => #lit_bstr => CMakeParse::parse(#tokens).map(|(parsed, tokens)| (Self::#ident(parsed), tokens)) }
            } else {
                quote_spanned! { ident.span() => #lit_bstr => Ok((Self::#ident, rest)) }
            }
        },
    )
}

fn enum_field_parsers(
    variants: &[CMakeEnum],
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    variants.iter().map(
        |CMakeEnum {
             option:
                 CMakeOption {
                     ident,
                     lit_bstr,
                     ..
                 },
             unnamed,
         }| {
            if *unnamed {
                quote_spanned! { ident.span() => CMakeParse::parse(tokens).map(|(parsed, tokens)| (Self::#ident(parsed), tokens)) }
            } else {
                quote_spanned! { ident.span() => Keyword::positional(#lit_bstr, tokens).map(|(_, tokens)| (Self::#ident, tokens)) }
            }
        },
    )
}

fn enum_fields(variants: &[CMakeEnum]) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    variants.iter().map(
        |CMakeEnum {
             option: CMakeOption {
                 ident, lit_bstr, ..
             },
             ..
         }| {
            quote_spanned! { ident.span() => #lit_bstr }
        },
    )
}

fn positional_var_defs(
    fields: &[CMakeOption],
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().enumerate().map(
        |(index, CMakeOption {
             ident, lit_bstr, ..
         })| {
            let def_mut = if index == fields.len() - 1 {
                quote! { mut }
            } else {
                quote! {}
            };
            quote_spanned! { ident.span() => let (#ident, #def_mut tokens) = CMakePositional::positional(#lit_bstr, tokens)? }
        },
    )
}

fn positional_fields(
    fields: &[CMakeOption],
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|CMakeOption { ident, .. }| {
        quote_spanned! { ident.span() => #ident }
    })
}

fn regular_var_defs(fields: &[CMakeOption]) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|CMakeOption { ident, .. }| {
        quote_spanned! { ident.span() => let mut #ident = CMakeParse::default_value() }
    })
}

fn regular_enum_defs(
    fields: &[CMakeOption],
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(
        |CMakeOption {
             ident, ident_mode, ..
         }| {
            quote_spanned! { ident.span() => #ident_mode }
        },
    )
}

fn regular_enum_match(
    fields: &[CMakeOption],
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(
        |CMakeOption {
             ident, ident_mode, ..
         }| {
            quote_spanned! { ident.span() => CMakeParserMode::#ident_mode => buffers.#ident.push(first.clone()) }
        },
    )
}

fn regular_fields(fields: &[CMakeOption]) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|CMakeOption { ident, lit_str, .. }| {
        quote_spanned! { ident.span() => #ident: #ident.end(&buffers.#ident)?.ok_or_else(|| CommandParseError::MissingToken(#lit_str.to_string()))? }
    })
}

fn regular_buf_fields(
    fields: &[CMakeOption],
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|CMakeOption { ident, .. }| {
        quote_spanned! { ident.span() => #ident: Vec<Token<'b>> }
    })
}

fn regular_if_stms(fields: &[CMakeOption]) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(
        |CMakeOption {
             ident,
             ident_mode,
             lit_bstr,
             ..
         }| {
            quote_spanned! { ident.span() => if #ident.matches(#lit_bstr, keyword) {
                let (update_mode, rest) = #ident.start(first, tokens, &mut buffers.#ident)?;
                tokens = rest;
                if update_mode {
                    current_mode = Some(CMakeParserMode::#ident_mode)
                };
            } else }
        },
    )
}

enum CMakeFields {
    StructNamedFields(Vec<CMakeOption>),
    EnumVariants(Vec<CMakeEnum>),
}
struct CMakeOption {
    id: String,
    attr: CMakeAttribute,
    ident: syn::Ident,
    ident_mode: syn::Ident,
    lit_str: proc_macro2::Literal,
    lit_bstr: proc_macro2::Literal,
}

impl CMakeOption {
    fn from_fields_named(fields_named: &syn::FieldsNamed) -> Vec<Self> {
        fields_named
            .named
            .iter()
            .filter_map(|f| {
                f.ident
                    .as_ref()
                    .map(|ident| (ident.clone(), cmake_attribute(&f.attrs).unwrap_or_default()))
            })
            .map(|(ident, attr)| {
                let id = ident.to_string();
                use inflections::Inflect;
                let ident_mode = quote::format_ident!("{}", id.to_pascal_case());
                let cmake_keyword = attr.rename.clone().unwrap_or_else(|| id.to_uppercase());
                let lit_str = proc_macro2::Literal::string(&cmake_keyword);
                let lit_bstr = proc_macro2::Literal::byte_string(cmake_keyword.as_bytes());
                CMakeOption {
                    id,
                    attr,
                    ident,
                    ident_mode,
                    lit_str,
                    lit_bstr,
                }
            })
            .collect()
    }
}

struct CMakeEnum {
    option: CMakeOption,
    unnamed: bool,
}

impl CMakeEnum {
    fn from_variants<'a>(variants: impl IntoIterator<Item = &'a syn::Variant>) -> Vec<Self> {
        variants
            .into_iter()
            .map(|f| {
                (
                    f.ident.clone(),
                    cmake_attribute(&f.attrs).unwrap_or_default(),
                    match &f.fields {
                        syn::Fields::Unit => false,
                        syn::Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => true,
                        _ => abort!(
                            f,
                            "only unit enums and unnamed enums with one field supported"
                        ),
                    },
                )
            })
            .map(|(ident, attr, unnamed)| {
                let id = ident.to_string();
                use inflections::Inflect;
                let ident_mode = quote::format_ident!("{}", id.to_pascal_case());
                let cmake_keyword = attr.rename.clone().unwrap_or_else(|| id.to_constant_case());
                let lit_str = proc_macro2::Literal::string(&cmake_keyword);
                let lit_bstr = proc_macro2::Literal::byte_string(cmake_keyword.as_bytes());
                CMakeEnum {
                    option: CMakeOption {
                        id,
                        attr,
                        ident,
                        ident_mode,
                        lit_str,
                        lit_bstr,
                    },
                    unnamed,
                }
            })
            .collect()
    }
}

struct CMakeImpl {
    ast: syn::DeriveInput,
    crate_path: proc_macro2::TokenStream,
}

impl CMakeImpl {
    fn new(ast: syn::DeriveInput, crate_path: proc_macro2::TokenStream) -> Self {
        Self { ast, crate_path }
    }

    fn trait_cmake_parse(&self, content: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let Self { ast, crate_path } = self;

        let name = &ast.ident;
        let generics = &ast.generics;
        let type_params = generics.type_params();
        let (_, ty_generics, where_clause) = generics.split_for_impl();

        quote! {
            #[automatically_derived]
            impl <'t #(, #type_params)*> #crate_path::CMakeParse<'t> for #name #ty_generics #where_clause {
                #content
            }
        }
    }

    fn trait_cmake_positional(
        &self,
        content: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let Self { ast, crate_path } = self;

        let name = &ast.ident;
        let generics = &ast.generics;
        let type_params = generics.type_params();
        let (_, ty_generics, where_clause) = generics.split_for_impl();

        quote! {
            #[automatically_derived]
            impl <'t #(, #type_params)*> #crate_path::CMakePositional<'t> for #name #ty_generics #where_clause {
                #content
            }
        }
    }

    fn fn_matches_type(&self, content: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        quote! {
            fn matches_type(_: &[u8], keyword: &[u8]) -> bool {
                #content
            }
        }
    }

    fn fn_need_push_keyword(&self, content: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;

        quote! {
            fn need_push_keyword(#[allow(unused_variables)] keyword: &#crate_path::Token<'t>) -> bool {
                #content
            }
        }
    }

    fn fn_parse(
        &self,
        is_mut: bool,
        content: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;
        let def_mut = if is_mut {
            quote! { mut }
        } else {
            quote! {}
        };

        quote! {
            fn parse<'tv>(
                #def_mut tokens: &'tv [#crate_path::Token<'t>],
            ) -> Result<(Self, &'tv [#crate_path::Token<'t>]), #crate_path::CommandParseError> {
                #content
            }
        }
    }

    fn fn_positional(&self, content: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;

        quote! {
            fn positional<'tv>(
                default_name: &'static [u8],
                tokens: &'tv [#crate_path::Token<'t>],
            ) -> Result<(Self, &'tv [#crate_path::Token<'t>]), #crate_path::CommandParseError> {
                #content
            }
        }
    }

    fn trait_cmake_positional_regular(&self) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;
        let fn_positional = self.fn_positional(quote! {
            #crate_path::CMakeParse::parse(tokens)
        });

        self.trait_cmake_positional(quote! {
            #fn_positional
        })
    }

    fn trait_cmake_parse_regular(&self, cmake_attr: CMakeAttribute) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;
        let fns_cmake = match self.to_cmake_fields() {
            CMakeFields::StructNamedFields(fields) => {
                let (positional_field_opts, regular_field_opts): (Vec<_>, Vec<_>) =
                    fields.into_iter().partition(|field| field.attr.positional);

                let has_regular_fields = !regular_field_opts.is_empty();

                let pos_var_defs = positional_var_defs(&positional_field_opts);
                let pos_fields = positional_fields(&positional_field_opts);

                let reg_var_defs = regular_var_defs(&regular_field_opts);
                let reg_fields = regular_fields(&regular_field_opts);
                let reg_buf_fields = regular_buf_fields(&regular_field_opts);
                let reg_enum_defs = regular_enum_defs(&regular_field_opts);
                let reg_enum_match = regular_enum_match(&regular_field_opts);
                let reg_if_stms = regular_if_stms(&regular_field_opts);

                let mode_default = cmake_attr
                    .default
                    .map(|def| {
                        use inflections::Inflect;

                        let defi = quote::format_ident!("{}", def.to_pascal_case());
                        quote! { Some(CMakeParserMode::#defi) }
                    })
                    .unwrap_or_else(|| {
                        quote! { None }
                    });

                let regular_fields = if has_regular_fields {
                    Some(quote! {
                        #[derive(Default)]
                        struct Buffers<'b> {
                            #(#reg_buf_fields,)*
                        }
                        enum CMakeParserMode {
                            #(#reg_enum_defs,)*
                        }
                        let mut buffers = Buffers::default();
                        let mut current_mode = #mode_default;

                        #(#reg_var_defs;)*

                        loop {
                            let Some((first, rest)) = tokens.split_first() else { break; };
                            tokens = rest;
                            let keyword = first.as_bytes();
                            #(#reg_if_stms)* {
                                match &current_mode {
                                    Some(mode) => match mode {
                                        #(#reg_enum_match,)*
                                    },
                                    None => {
                                        return Err(CommandParseError::UnknownOption(
                                            String::from_utf8_lossy(keyword).to_string(),
                                        ))
                                    }
                                }
                            }
                        }
                    })
                } else {
                    None
                };

                let fn_parse = self.fn_parse(
                    positional_field_opts.is_empty(),
                    quote! {
                        use #crate_path::{CommandParseError, CMakeParse, CMakePositional, Token};

                        #(#pos_var_defs;)*

                        #regular_fields

                        Ok((Self {
                            #(#pos_fields,)*
                            #(#reg_fields,)*
                        }, tokens))
                    },
                );

                quote! {
                    #fn_parse
                }
            }
            CMakeFields::EnumVariants(variants) => {
                if cmake_attr.untagged {
                    self.trait_cmake_parse_enum_untagged(variants)
                } else {
                    self.trait_cmake_parse_enum_tagged(variants)
                }
            }
        };

        self.trait_cmake_parse(quote! {
            #fns_cmake
        })
    }

    fn trait_cmake_parse_positional(&self) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;
        let CMakeFields::StructNamedFields(struct_named_fields) = self.to_cmake_fields() else {
            abort!(self.ast.ident, "positional top level attribute allowed only for structs with named fields.");
        };

        let var_defs = positional_var_defs(&struct_named_fields);

        let fields = positional_fields(&struct_named_fields);

        let fn_cmake_parse = self.fn_parse(
            false,
            quote! {
                use #crate_path::CMakePositional;
                #(#var_defs;)*
                Ok((Self {
                    #(#fields,)*
                }, tokens))
            },
        );

        self.trait_cmake_parse(quote! {
            #fn_cmake_parse
        })
    }

    fn to_cmake_fields(&self) -> CMakeFields {
        let name = &self.ast.ident;

        match &self.ast.data {
            syn::Data::Struct(data_struct) => match &data_struct.fields {
                syn::Fields::Named(fields_named) => {
                    CMakeFields::StructNamedFields(CMakeOption::from_fields_named(fields_named))
                }
                syn::Fields::Unnamed(_) => {
                    abort!(data_struct.fields, "unnamed fields are not supported")
                }
                syn::Fields::Unit => {
                    abort!(name, "unit fields are not supported")
                }
            },
            syn::Data::Enum(DataEnum { variants, .. }) => {
                CMakeFields::EnumVariants(CMakeEnum::from_variants(variants))
            }
            syn::Data::Union(_) => {
                abort!(name, "unions are not supported")
            }
        }
    }

    fn trait_cmake_parse_enum_tagged(&self, variants: Vec<CMakeEnum>) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;

        let enum_flds = enum_fields(&variants);
        let fn_matches_type = self.fn_matches_type(quote! {
            const FIELDS: &[&[u8]] = &[#(#enum_flds),*];
            FIELDS.contains(&keyword)
        });

        let enum_fld_matches = enum_field_matches(&variants);
        let fn_parse = self.fn_parse(
            false,
            quote! {
                use #crate_path::{CommandParseError, CMakeParse, CMakePositional, Token};
                let Some((enum_member, rest)) = tokens.split_first() else {
                    return Err(CommandParseError::TokenRequired);
                };

                match enum_member.as_bytes() {
                    #(#enum_fld_matches,)*
                    keyword => Err(CommandParseError::UnknownOption(
                        String::from_utf8_lossy(keyword).to_string(),
                    )),
                }
            },
        );

        let fn_need_push_keyword = self.fn_need_push_keyword(quote! {
            true
        });

        quote! {
            #fn_matches_type
            #fn_parse
            #fn_need_push_keyword
        }
    }

    fn trait_cmake_parse_enum_untagged(
        &self,
        variants: Vec<CMakeEnum>,
    ) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;

        let enum_fld_parsers = enum_field_parsers(&variants);
        let fn_parse = self.fn_parse(
            false,
            quote! {
                use #crate_path::{CMakeParse, CMakePositional, Keyword};
                Err(#crate_path::CommandParseError::TokenRequired)
                #(.or_else(|_| #enum_fld_parsers))*
            },
        );

        quote! {
            #fn_parse
        }
    }
}

#[derive(Default)]
struct CMakeAttribute {
    default: Option<String>,
    positional: bool,
    transparent: bool,
    untagged: bool,

    pkg: Option<syn::Path>,
    rename: Option<String>,
}

fn cmake_attribute(attrs: &[syn::Attribute]) -> Option<CMakeAttribute> {
    let attr = attrs.iter().find(|attr| attr.path().is_ident("cmake"))?;

    let nested = attr
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        .unwrap();

    let mut rename = None;
    let mut pkg = None;
    let mut transparent = false;
    let mut positional = false;
    let mut untagged = false;
    let mut default = None;

    for meta in nested {
        match meta {
            Meta::Path(p) if p.is_ident("transparent") => transparent = true,
            Meta::Path(p) if p.is_ident("positional") => positional = true,
            Meta::Path(p) if p.is_ident("untagged") => untagged = true,
            Meta::NameValue(MetaNameValue {
                ref path,
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }),
                ..
            }) => {
                if path.is_ident("default") {
                    default = Some(s.value());
                } else if path.is_ident("rename") {
                    rename = Some(s.value());
                } else if path.is_ident("pkg") {
                    pkg = s.parse().ok();
                }
            }
            _ => (),
        }
    }

    Some(CMakeAttribute {
        pkg,
        rename,
        default,
        positional,
        transparent,
        untagged,
    })
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, Attribute};

    use super::*;

    #[test]
    fn enum_ast() {
        let en: syn::Stmt = parse_quote! {
            enum Test {
                Var1,
                Var2(String),
                Var3 { value: String }
            }
        };
        dbg!(en);
    }
    #[test]
    fn check_def_attr() {
        let attr: Attribute = parse_quote! {
            #[cmake(default = "COMMAND",
                rename = "mmm",
                pkg = "crate",
                transparent,
                positional
            )]
        };

        let cmake_attr = cmake_attribute(&[attr]).expect("attrs");
        assert!(cmake_attr.pkg.is_some());
        assert_eq!(Some("mmm"), cmake_attr.rename.as_deref());
        assert_eq!(Some("COMMAND"), cmake_attr.default.as_deref());
        assert!(cmake_attr.positional);
        assert!(cmake_attr.transparent);
    }
}
