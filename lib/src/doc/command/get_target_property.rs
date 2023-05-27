use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Get a property from a target.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/get_target_property.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct GetTargetProperty<'t> {
    pub var: Token<'t>,
    pub target: Token<'t>,
    pub property: Token<'t>,
}

impl<'t> ToCommandScope for GetTargetProperty<'t> {
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
    fn get_target_property() {
        let src = include_bytes!("../../../../fixture/commands/get_target_property");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::GetTargetProperty(Box::new(
                GetTargetProperty {
                    var: token(b"linkedLibs"),
                    target: token(b"${target}"),
                    property: token(b"LINK_LIBRARIES"),
                }
            )),])
        )
    }
}
