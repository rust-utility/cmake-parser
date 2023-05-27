use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Get a property of the test.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/get_test_property.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct GetTestProperty<'t> {
    pub test: Token<'t>,
    pub property: Token<'t>,
    pub var: Token<'t>,
}

impl<'t> ToCommandScope for GetTestProperty<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_test_property() {
        let src = include_bytes!("../../../../fixture/commands/get_test_property");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::GetTestProperty(Box::new(GetTestProperty {
                test: token(b"testX"),
                property: token(b"DEPENDS"),
                var: token(b"DependenciesOfTestX"),
            })),])
        )
    }
}
