#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::Punctuated, DataEnum, DeriveInput, Expr, ExprArray, ExprLit, Lit, Meta,
    MetaNameValue, Token,
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

    let positional = cmake_attr.positional;
    let cmake_impl = CMakeImpl::new(ast, cmake_parse_path, cmake_attr);
    let trait_cmake_parse = if positional {
        cmake_impl.trait_cmake_parse_positional()
    } else {
        cmake_impl.trait_cmake_parse_regular()
    };

    let trait_cmake_positional = cmake_impl.trait_cmake_positional_regular();

    quote! {
        #trait_cmake_parse
        #trait_cmake_positional
    }
    .into()
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
    has_keyword: bool,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().enumerate().map(
        move |(index, CMakeOption {
             ident, lit_bstr, attr: CMakeAttribute { transparent , keyword_after, in_range, last, allow_empty, ..}, ..
         })| {
            let def_mut = if index == fields.len() - 1 {
                quote! { mut }
            } else {
                quote! {}
            };
            let has_keyword = has_keyword || *transparent;
            let tokens = if *last {
                quote! { last }
            } else {
                quote! { tokens }
            };
            let keyword_after = keyword_after.as_ref().map(|bstr| { quote! { ; let (_, #def_mut #tokens) = Keyword::positional(#bstr, #tokens, false)? } });
            if *in_range && index != fields.len() - 1 {
                let allow_empty = *allow_empty;
                let range_to_keyword = &fields[index + 1].lit_bstr;
                quote_spanned! { ident.span() => let (#ident, #def_mut #tokens) = CMakePositional::in_range(#lit_bstr, #range_to_keyword, #allow_empty, #tokens, #has_keyword)? #keyword_after }
            } else {
                quote_spanned! { ident.span() => let (#ident, #def_mut #tokens) = CMakePositional::positional(#lit_bstr, #tokens, #has_keyword)? #keyword_after }
            }
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
            quote_spanned! { ident.span() => CMakeParserMode::#ident_mode => #ident.push_keyword(&mut buffers.#ident, first) }
        },
    )
}

fn regular_fields(fields: &[CMakeOption]) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|CMakeOption { ident, lit_str, .. }| {
        quote_spanned! { ident.span() => #ident: #ident.end(&buffers.#ident)?.ok_or_else(|| CommandParseError::MissingToken(#lit_str.to_string()))? }
    })
}

fn regular_match_fields(
    fields: &[CMakeOption],
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(
        |CMakeOption {
             ident, lit_bstr, ty, ..
         }| {
            quote_spanned! { ident.span() => <#ty as CMakeParse>::matches_type(#lit_bstr, keyword, tokens) }
        },
    )
}

fn regular_match_fields_need_update(
    fields: &[CMakeOption],
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(
        |CMakeOption {
             ident, lit_bstr, ty, ..
         }| {
            quote_spanned! { ident.span() => <#ty as CMakeParse>::matches_type(#lit_bstr, keyword_bytes, &[]) && buffer.iter().any(|token| <#ty as CMakeParse>::matches_type(#lit_bstr, token.as_bytes(), &[])) }
        },
    )
}

fn regular_buf_fields(
    fields: &[CMakeOption],
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|CMakeOption { ident, .. }| {
        quote_spanned! { ident.span() => #ident: Vec<Token<'b>> }
    })
}

fn regular_if_stms(
    fields: &[CMakeOption],
    mode_default: proc_macro2::TokenStream,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(
        move |CMakeOption {
                  ident,
                  ident_mode,
                  lit_bstr,
                  ..
              }| {
            quote_spanned! { ident.span() => if #ident.matches(#lit_bstr, keyword, tokens) {
                let (update_mode, rest) = #ident.start(#lit_bstr, first, tokens, &mut buffers.#ident)?;
                tokens = rest;
                current_mode = if update_mode {
                    Some(CMakeParserMode::#ident_mode)
                } else {
                    #[allow(clippy::no_effect)]
                    ();
                    #mode_default
                };
            } else }
        },
    )
}

enum CMakeFields {
    StructNamedFields(Vec<CMakeOption>),
    EnumVariants(Vec<CMakeEnum>),
    Unit,
}
struct CMakeOption {
    attr: CMakeAttribute,
    ident: syn::Ident,
    ident_mode: syn::Ident,
    lit_str: proc_macro2::Literal,
    lit_bstr: proc_macro2::Literal,
    ty: Option<syn::Type>,
}

impl CMakeOption {
    fn from_fields_named(fields_named: &syn::FieldsNamed) -> Vec<Self> {
        fields_named
            .named
            .iter()
            .filter_map(|f| {
                f.ident.as_ref().map(|ident| {
                    (
                        ident.clone(),
                        f.ty.clone(),
                        cmake_attribute(&f.attrs).unwrap_or_default(),
                    )
                })
            })
            .map(|(ident, ty, attr)| {
                let id = ident.to_string();
                use inflections::Inflect;
                let ident_mode = quote::format_ident!("{}", id.to_pascal_case());
                let cmake_keyword = attr.rename.clone().unwrap_or_else(|| id.to_uppercase());
                let lit_str = proc_macro2::Literal::string(&cmake_keyword);
                let lit_bstr = proc_macro2::Literal::byte_string(cmake_keyword.as_bytes());
                CMakeOption {
                    attr,
                    ident,
                    ident_mode,
                    lit_str,
                    lit_bstr,
                    ty: Some(ty),
                }
            })
            .collect()
    }
}

struct StrBStr {
    lit_bstr: proc_macro2::Literal,
}

struct CMakeEnum {
    option: CMakeOption,
    renames: Option<Vec<StrBStr>>,
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
                    renames: attr.renames.as_deref().map(|keywords| {
                        keywords
                            .iter()
                            .map(|keyword| StrBStr {
                                lit_bstr: proc_macro2::Literal::byte_string(keyword.as_bytes()),
                            })
                            .collect()
                    }),
                    option: CMakeOption {
                        attr,
                        ident,
                        ident_mode,
                        lit_str,
                        lit_bstr,
                        ty: None,
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
    cmake_attr: CMakeAttribute,
}

impl CMakeImpl {
    fn new(
        ast: syn::DeriveInput,
        crate_path: proc_macro2::TokenStream,
        cmake_attr: CMakeAttribute,
    ) -> Self {
        Self {
            ast,
            crate_path,
            cmake_attr,
        }
    }

    fn trait_cmake_parse(&self, content: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let Self {
            ast, crate_path, ..
        } = self;

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
        let Self {
            ast, crate_path, ..
        } = self;

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
        let Self { crate_path, .. } = self;
        quote! {
            fn matches_type(_: &[u8], keyword: &[u8], tokens: &[#crate_path::Token<'t>]) -> bool {
                #content
            }
        }
    }

    fn fn_need_update(&self, content: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let Self { crate_path, .. } = self;
        quote! {
            fn need_update(field_keyword: &[u8], keyword: &#crate_path::Token<'t>, buffer: &[#crate_path::Token<'t>]) -> bool {
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
                mut tokens: &'tv [#crate_path::Token<'t>],
                has_keyword: bool,
            ) -> Result<(Self, &'tv [#crate_path::Token<'t>]), #crate_path::CommandParseError> {
                #content
            }
        }
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
                syn::Fields::Unit => CMakeFields::Unit,
            },
            syn::Data::Enum(DataEnum { variants, .. }) => {
                CMakeFields::EnumVariants(CMakeEnum::from_variants(variants))
            }
            syn::Data::Union(_) => {
                abort!(name, "unions are not supported")
            }
        }
    }

    fn trait_cmake_positional_regular(&self) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;
        let fn_positional = self.fn_positional(quote! {
            if has_keyword {
                let (_, rest) = #crate_path::Keyword::positional(default_name, tokens, has_keyword)?;
                tokens = rest;
            }
            #crate_path::CMakeParse::parse(tokens)
        });

        self.trait_cmake_positional(quote! {
            #fn_positional
        })
    }

    fn trait_cmake_parse_regular(&self) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;
        let fns_cmake = match self.to_cmake_fields() {
            CMakeFields::StructNamedFields(fields) => {
                let (positional_field_opts, regular_field_opts): (Vec<_>, Vec<_>) =
                    fields.into_iter().partition(|field| field.attr.positional);

                let has_regular_fields = !regular_field_opts.is_empty();

                let pos_var_defs =
                    positional_var_defs(&positional_field_opts, self.cmake_attr.transparent);
                let pos_fields = positional_fields(&positional_field_opts);

                let reg_var_defs = regular_var_defs(&regular_field_opts);
                let reg_fields = regular_fields(&regular_field_opts);
                let reg_buf_fields = regular_buf_fields(&regular_field_opts);
                let reg_enum_defs = regular_enum_defs(&regular_field_opts);
                let reg_enum_match = regular_enum_match(&regular_field_opts);

                let mode_default = self
                    .cmake_attr
                    .default
                    .as_deref()
                    .map(|def| {
                        use inflections::Inflect;

                        let defi = quote::format_ident!("{}", def.to_pascal_case());
                        quote! { Some(CMakeParserMode::#defi) }
                    })
                    .unwrap_or_else(|| {
                        quote! { None }
                    });

                let reg_if_stms = regular_if_stms(&regular_field_opts, mode_default.clone());
                let reg_except_if_stmt = self.regular_except_if_stmt();

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
                            #reg_except_if_stmt #(#reg_if_stms)* {
                                match &current_mode {
                                    Some(cmake_active_mode) => {
                                        if match cmake_active_mode {
                                            #(#reg_enum_match,)*
                                        } {
                                            current_mode = #mode_default;
                                        }
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

                let check_empty = if !self.cmake_attr.allow_empty {
                    Some(quote! {
                        if tokens.is_empty() {
                            return Err(CommandParseError::TokenRequired);
                        }
                    })
                } else {
                    None
                };
                let require_empty = if self.cmake_attr.complete {
                    Some(quote! {
                        if !tokens.is_empty() {
                            return Err(#crate_path::CommandParseError::Incomplete);
                        }
                    })
                } else {
                    None
                };

                let fn_parse = self.fn_parse(
                    positional_field_opts.is_empty(),
                    quote! {
                        use #crate_path::{CommandParseError, CMakeParse, CMakePositional, Keyword, Token};
                        #check_empty

                        #(#pos_var_defs;)*

                        #regular_fields

                        #require_empty

                        Ok((Self {
                            #(#pos_fields,)*
                            #(#reg_fields,)*
                        }, tokens))
                    },
                );

                let fns_for_match_fields = if self.cmake_attr.match_fields {
                    let reg_match_fields = regular_match_fields(&regular_field_opts);
                    let fn_matches_type = self.fn_matches_type(quote! {
                        use #crate_path::CMakeParse;
                        #(#reg_match_fields)||*
                    });

                    let reg_match_fields_need_update =
                        regular_match_fields_need_update(&regular_field_opts);
                    let fn_need_update = self.fn_need_update(quote! {
                        use #crate_path::CMakeParse;
                        let keyword_bytes = keyword.as_bytes();
                        buffer.contains(keyword)
                        #(|| (#reg_match_fields_need_update))*
                    });

                    let fn_need_push_keyword = self.fn_need_push_keyword(quote! {
                        true
                    });

                    quote! {
                        #fn_matches_type
                        #fn_need_update
                        #fn_need_push_keyword
                    }
                } else {
                    quote! {}
                };

                quote! {
                    #fn_parse
                    #fns_for_match_fields
                }
            }
            CMakeFields::EnumVariants(variants) => self.trait_cmake_parse_enum(&variants),
            CMakeFields::Unit => {
                let fn_parse = self.fn_parse(
                    false,
                    quote! {
                        if tokens.is_empty() {
                            Ok((Self, tokens))
                        } else {
                            Err(#crate_path::CommandParseError::NotEmpty)
                        }
                    },
                );
                quote! {
                    #fn_parse
                }
            }
        };

        self.trait_cmake_parse(quote! {
            #fns_cmake
        })
    }

    fn trait_cmake_parse_positional(&self) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;
        let fns_cmake = match self.to_cmake_fields() {
            CMakeFields::StructNamedFields(struct_named_fields) => {
                let split_last_count = struct_named_fields.iter().filter(|f| f.attr.last).count();
                let split_last = if split_last_count > 0 {
                    Some(quote! {
                        let Some((tokens, last)) = tokens.split_last_chunk::<#split_last_count>() else {
                            return Err(#crate_path::CommandParseError::TokenRequired);
                        };
                    })
                } else {
                    None
                };
                let var_defs =
                    positional_var_defs(&struct_named_fields, self.cmake_attr.transparent);

                let fields = positional_fields(&struct_named_fields);

                let check_empty = if self.cmake_attr.complete {
                    Some(quote! {
                        if !tokens.is_empty() {
                            return Err(#crate_path::CommandParseError::Incomplete);
                        }
                    })
                } else {
                    None
                };

                let fn_cmake_parse = self.fn_parse(
                    false,
                    quote! {
                        use #crate_path::{CMakePositional, Keyword};
                        #split_last
                        #(#var_defs;)*
                        #check_empty
                        Ok((Self {
                            #(#fields,)*
                        }, tokens))
                    },
                );

                quote! {
                    #fn_cmake_parse
                }
            }
            CMakeFields::EnumVariants(variants) => self.trait_cmake_parse_enum(&variants),
            CMakeFields::Unit => abort!(
                self.ast.ident,
                "positional top level attribute not allowed for structs with unit fields."
            ),
        };
        self.trait_cmake_parse(quote! {
            #fns_cmake
        })
    }

    fn trait_cmake_parse_enum(&self, variants: &[CMakeEnum]) -> proc_macro2::TokenStream {
        if self.cmake_attr.untagged {
            self.trait_cmake_parse_enum_untagged(variants)
        } else {
            self.trait_cmake_parse_enum_tagged(variants)
        }
    }

    fn trait_cmake_parse_enum_tagged(&self, variants: &[CMakeEnum]) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;

        let fn_matches_type = if !self.cmake_attr.list {
            let enum_flds = enum_fields(variants);
            Some(self.fn_matches_type(quote! {
                const FIELDS: &[&[u8]] = &[#(#enum_flds),*];
                FIELDS.contains(&keyword)
            }))
        } else {
            None
        };

        let enum_fld_matches = self.enum_field_matches(variants);
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

        let fn_need_update = if self.cmake_attr.complete {
            Some(self.fn_need_update(quote! { false }))
        } else {
            None
        };

        let fn_need_push_keyword = if !self.cmake_attr.list {
            Some(self.fn_need_push_keyword(quote! {
                true
            }))
        } else {
            None
        };

        quote! {
            #fn_matches_type
            #fn_parse
            #fn_need_update
            #fn_need_push_keyword
        }
    }

    fn trait_cmake_parse_enum_untagged(&self, variants: &[CMakeEnum]) -> proc_macro2::TokenStream {
        let crate_path = &self.crate_path;

        let enum_fld_parsers = self.enum_field_parsers(variants);
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

    fn enum_field_parsers<'v>(
        &self,
        variants: &'v [CMakeEnum],
    ) -> impl Iterator<Item = proc_macro2::TokenStream> + 'v {
        let attr_transparent = self.cmake_attr.transparent;
        let attr_complete = self.cmake_attr.complete;
        variants.iter().map(
            move |CMakeEnum {
                 option:
                     CMakeOption {
                         ident,
                         lit_bstr,
                         attr: CMakeAttribute {
                            complete,
                            transparent, ..
                         },
                         ..
                     },
                 renames,
                 unnamed,
             }| {
                let transparent = *transparent || attr_transparent;
                let complete = *complete || attr_complete;
                let lit_bstrs = renames.as_ref().map(|strbstrs| strbstrs.iter().map(|strbstr| &strbstr.lit_bstr).collect()).unwrap_or_else(|| vec![lit_bstr]);
                let positional = format_ident!("{}", if !complete { "positional" } else { "positional_complete"});
                if *unnamed {
                    quote_spanned! { ident.span() => #(CMakePositional::#positional(#lit_bstrs, tokens, #transparent).map(|(parsed, tokens)| (Self::#ident(parsed), tokens))),* }
                } else {
                    quote_spanned! { ident.span() => #(Keyword::#positional(#lit_bstrs, tokens, #transparent).map(|(_, tokens)| (Self::#ident, tokens))),* }
                }
            },
        )
    }

    fn enum_field_matches<'v>(
        &self,
        variants: &'v [CMakeEnum],
    ) -> impl Iterator<Item = proc_macro2::TokenStream> + 'v {
        let enum_transparent = self.cmake_attr.transparent;
        let enum_positional = self.cmake_attr.positional;
        variants.iter().map(
            move |CMakeEnum {
                 option:
                     CMakeOption {
                         ident,
                         lit_bstr,
                         attr: CMakeAttribute {
                            transparent,
                            positional,
                            ..
                         },
                         ..
                     },
                     renames,
                     unnamed,
             }| {
                let positional = enum_positional || *positional;


                let tokens = if enum_transparent || *transparent {
                    quote! { rest }
                } else {
                    quote! { tokens }
                };
                let parser = if positional {
                    quote! { CMakePositional::positional(#lit_bstr, #tokens, false) }
                } else {
                    quote! { CMakeParse::parse(#tokens) }
                };
                let lit_bstrs = renames.as_ref().map(|strbstrs| strbstrs.iter().map(|strbstr| &strbstr.lit_bstr).collect()).unwrap_or_else(|| vec![lit_bstr]);
                if *unnamed {
                    quote_spanned! { ident.span() => #(#lit_bstrs)|* => #parser.map(|(parsed, tokens)| (Self::#ident(parsed), tokens)) }
                } else {
                    quote_spanned! { ident.span() => #(#lit_bstrs)|* => Ok((Self::#ident, rest)) }
                }
            },
        )
    }

    fn regular_except_if_stmt(&self) -> Option<proc_macro2::TokenStream> {
        self.cmake_attr.except.as_deref().map(|except| {
            let except = except
                .iter()
                .map(|e| proc_macro2::Literal::byte_string(e.as_bytes()));
            let crate_path = &self.crate_path;
            quote! {
                const FIELDS: &[&[u8]] = &[#(#except),*];
                if FIELDS.contains(&keyword) {
                    return Err(#crate_path::CommandParseError::Incomplete)
                } else
            }
        })
    }
}

#[derive(Default)]
struct CMakeAttribute {
    default: Option<String>,
    keyword_after: Option<proc_macro2::Literal>,
    list: bool,
    match_fields: bool,
    pkg: Option<syn::Path>,
    positional: bool,
    rename: Option<String>,
    renames: Option<Vec<String>>,
    transparent: bool,
    untagged: bool,
    allow_empty: bool,
    complete: bool,
    except: Option<Vec<String>>,
    in_range: bool,
    last: bool,
}

fn cmake_attribute(attrs: &[syn::Attribute]) -> Option<CMakeAttribute> {
    let attr = attrs.iter().find(|attr| attr.path().is_ident("cmake"))?;

    let nested = attr
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        .unwrap();

    let mut default = None;
    let mut keyword_after = None;
    let mut list = false;
    let mut match_fields = false;
    let mut pkg = None;
    let mut positional = false;
    let mut rename = None;
    let mut renames = None;
    let mut transparent = false;
    let mut untagged = false;
    let mut allow_empty = false;
    let mut complete = false;
    let mut except = None;
    let mut in_range = false;
    let mut last = false;

    for meta in nested {
        match meta {
            Meta::Path(p) if p.is_ident("list") => list = true,
            Meta::Path(p) if p.is_ident("match_fields") => match_fields = true,
            Meta::Path(p) if p.is_ident("positional") => positional = true,
            Meta::Path(p) if p.is_ident("transparent") => transparent = true,
            Meta::Path(p) if p.is_ident("untagged") => untagged = true,
            Meta::Path(p) if p.is_ident("allow_empty") => allow_empty = true,
            Meta::Path(p) if p.is_ident("complete") => complete = true,
            Meta::Path(p) if p.is_ident("in_range") => in_range = true,
            Meta::Path(p) if p.is_ident("last") => last = true,
            Meta::NameValue(MetaNameValue {
                ref path,
                value: Expr::Array(ExprArray { elems, .. }),
                ..
            }) => {
                if path.is_ident("rename") {
                    renames = Some(to_vec_string(elems));
                } else if path.is_ident("except") {
                    except = Some(to_vec_string(elems));
                }
            }
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
                } else if path.is_ident("keyword_after") {
                    keyword_after = Some(proc_macro2::Literal::byte_string(s.value().as_bytes()));
                } else if path.is_ident("pkg") {
                    pkg = s.parse().ok();
                } else if path.is_ident("rename") {
                    rename = Some(s.value());
                }
            }
            _ => (),
        }
    }

    Some(CMakeAttribute {
        default,
        keyword_after,
        list,
        match_fields,
        pkg,
        positional,
        rename,
        renames,
        transparent,
        untagged,
        allow_empty,
        complete,
        except,
        in_range,
        last,
    })
}

fn to_vec_string(elems: Punctuated<Expr, syn::token::Comma>) -> Vec<String> {
    elems
        .iter()
        .filter_map(|elem| match elem {
            Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) => Some(s.value()),
            _ => None,
        })
        .collect()
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
                positional,
                match_fields,
                list,
            )]
        };

        let cmake_attr = cmake_attribute(&[attr]).expect("attrs");
        assert!(cmake_attr.pkg.is_some());
        assert_eq!(Some("mmm"), cmake_attr.rename.as_deref());
        assert_eq!(Some("COMMAND"), cmake_attr.default.as_deref());
        assert!(cmake_attr.positional);
        assert!(cmake_attr.transparent);
        assert!(cmake_attr.match_fields);
        assert!(cmake_attr.list);
    }

    #[test]
    fn check_attr_rename() {
        let attr: Attribute = parse_quote! {
            #[cmake(
                rename = ["aaa", "bb", "c"]
            )]
        };

        let cmake_attr = cmake_attribute(&[attr]).expect("attrs");
        assert_eq!(
            Some(vec!["aaa".to_string(), "bb".to_string(), "c".to_string()]),
            cmake_attr.renames
        );
    }
    #[test]
    fn check_attr_except() {
        let attr: Attribute = parse_quote! {
            #[cmake(
                except = ["aaa", "bb", "c"]
            )]
        };

        let cmake_attr = cmake_attribute(&[attr]).expect("attrs");
        assert_eq!(
            Some(vec!["aaa".to_string(), "bb".to_string(), "c".to_string()]),
            cmake_attr.except
        );
    }
}
