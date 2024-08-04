use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Get a global property of the CMake instance.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/get_cmake_property.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct GetCMakeProperty<'t> {
    pub var: Token<'t>,
    pub property: Token<'t>,
}

impl<'t> ToCommandScope for GetCMakeProperty<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_cmake_property() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/get_cmake_property");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![Ok(Command::GetCMakeProperty(Box::new(GetCMakeProperty {
                var: token(b"var1"),
                property: token(b"property1"),
            }))),]
        )
    }
}
