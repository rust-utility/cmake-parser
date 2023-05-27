use ::cmake_parser_derive::CMake;

use crate::{CommandScope, Keyword, ToCommandScope, Token};

/// Define and document custom properties.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/define_property.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct DefineProperty<'t> {
    #[cmake(positional)]
    pub property_scope: PropertyScope,
    #[cmake(positional)]
    property: Keyword,
    #[cmake(positional)]
    pub property_name: Token<'t>,
    pub inherited: bool,
    pub brief_docs: Option<Vec<Token<'t>>>,
    pub full_docs: Option<Vec<Token<'t>>>,
    pub initialize_from_variable: Option<Token<'t>>,
}

impl<'t> ToCommandScope for DefineProperty<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum PropertyScope {
    Global,
    Directory,
    Target,
    Source,
    Test,
    Variable,
    CachedVariable,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::quoted_tokens;
    use crate::*;

    #[test]
    fn define_property() {
        let src = include_bytes!("../../../../../fixture/commands/project/define_property");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);

        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::DefineProperty(Box::new(DefineProperty {
                    property_scope: PropertyScope::Target,
                    property: Keyword,
                    property_name: b"EXAMPLE_TYPE".into(),
                    inherited: false,
                    brief_docs: Some(
                        quoted_tokens([b"Whether given target describes example."]).to_vec()
                    ),
                    full_docs: Some(
                        quoted_tokens([b"Whether given target describes example."]).to_vec()
                    ),
                    initialize_from_variable: None,
                })),
                Command::DefineProperty(Box::new(DefineProperty {
                    property_scope: PropertyScope::CachedVariable,
                    property: Keyword,
                    property_name: b"A_PROPERTY".into(),
                    inherited: false,
                    brief_docs: Some(quoted_tokens([b"brief"]).to_vec()),
                    full_docs: Some(quoted_tokens([b"full"]).to_vec()),
                    initialize_from_variable: None,
                })),
            ])
        )
    }
}
