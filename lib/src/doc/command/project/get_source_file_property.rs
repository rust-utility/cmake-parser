use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Get a property for a source file.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/get_source_file_property.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct GetSourceFileProperty<'t> {
    pub variable: Token<'t>,
    pub file: Token<'t>,
    pub source: PropertySource<'t>,
    pub property: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum PropertySource<'t> {
    Directory(Token<'t>),
    TargetDirectory(Token<'t>),
}

impl<'t> ToCommandScope for GetSourceFileProperty<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{quoted_token, token};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_source_file_property() {
        let src =
            include_bytes!("../../../../../fixture/commands/project/get_source_file_property");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands().unwrap(),
            &[Command::GetSourceFileProperty(Box::new(
                GetSourceFileProperty {
                    variable: token(b"_generated"),
                    file: quoted_token(b"${_generated_candidate}"),
                    source: PropertySource::TargetDirectory(quoted_token(b"${target}")),
                    property: token(b"GENERATED"),
                }
            )),]
        )
    }
}
