use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add include directories to a target.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/target_include_directories.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct TargetIncludeDirectories<'t> {
    #[cmake(positional)]
    pub name: Token<'t>,
}

impl<'t> ToCommandScope for TargetIncludeDirectories<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn target_include_directories() {
        let src =
            include_bytes!("../../../../../fixture/commands/project/target_include_directories");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::TargetIncludeDirectories(Box::new(
                TargetIncludeDirectories {
                    name: token(b"name"),
                }
            )),])
        )
    }
}
