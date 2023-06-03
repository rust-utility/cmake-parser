use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Sets properties associated with source files using a key/value paired list.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/set_source_files_properties.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "files")]
pub struct SetSourceFileProperties<'t> {
    #[cmake(rename = "")]
    pub files: Vec<Token<'t>>,
    #[cmake(rename = "DIRECTORY")]
    pub directories: Option<Vec<Token<'t>>>,
    #[cmake(rename = "TARGET_DIRECTORY")]
    pub targets: Option<Vec<Token<'t>>>,
    pub properties: Vec<Property<'t>>,
}

impl<'t> ToCommandScope for SetSourceFileProperties<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Property<'t> {
    pub prop: Token<'t>,
    pub value: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{quoted_token, token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn set_source_files_properties() {
        let src =
            include_bytes!("../../../../../fixture/commands/project/set_source_files_properties");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::SetSourceFileProperties(Box::new(SetSourceFileProperties {
                    files: tokens_vec([b"a.cpp"]),
                    directories: None,
                    targets: None,
                    properties: vec![Property {
                        prop: token(b"COMPILE_DEFINITIONS"),
                        value: quoted_token(b"DIR1=/home/x x/b.i;DIR2=/home/xxx/c.i"),
                    }]
                })),
                Command::SetSourceFileProperties(Box::new(SetSourceFileProperties {
                    files: tokens_vec([b"example.i", b"example.q"]),
                    directories: Some(tokens_vec([b"qqq1", b"qqq2"])),
                    targets: Some(tokens_vec([b"ddd1", b"ddd2"])),
                    properties: vec![
                        Property {
                            prop: token(b"CPLUSPLUS"),
                            value: token(b"ON"),
                        },
                        Property {
                            prop: token(b"SWIG_FLAGS"),
                            value: quoted_token(b"-includeall"),
                        }
                    ]
                })),
            ])
        )
    }
}
