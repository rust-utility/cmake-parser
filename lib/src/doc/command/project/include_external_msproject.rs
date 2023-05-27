use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Include an external Microsoft project file in a workspace.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/include_external_msproject.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "deps")]
pub struct IncludeExternalMSProject<'t> {
    #[cmake(positional)]
    pub project_name: Token<'t>,
    #[cmake(positional)]
    pub location: Token<'t>,
    #[cmake(rename = "TYPE")]
    pub project_type: Option<Token<'t>>,
    pub guid: Option<Token<'t>>,
    pub platform: Option<Token<'t>>,
    #[cmake(rename = "")]
    pub deps: Option<Vec<Token<'t>>>,
}

impl<'t> ToCommandScope for IncludeExternalMSProject<'t> {
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
    fn include_external_msproject() {
        let src =
            include_bytes!("../../../../../fixture/commands/project/include_external_msproject");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::IncludeExternalMSProject(Box::new(
                IncludeExternalMSProject {
                    project_name: token(b"splash"),
                    location: token(b"${CMAKE_SOURCE_DIR}/splash.vcxproj"),
                    project_type: None,
                    guid: None,
                    platform: Some(token(b"Win32")),
                    deps: None,
                }
            )),])
        )
    }
}
