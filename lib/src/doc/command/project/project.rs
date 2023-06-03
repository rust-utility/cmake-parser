use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Set the name of the project.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/project.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Project<'t> {
    pub project_name: Token<'t>,
    pub details: Option<ProjectDetails<'t>>,
}

impl<'t> ToCommandScope for Project<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum ProjectDetails<'t> {
    General(GeneralProjectDetails<'t>),
    Short(Vec<Token<'t>>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct GeneralProjectDetails<'t> {
    pub version: Option<Token<'t>>,
    pub description: Option<Token<'t>>,
    pub homepage_url: Option<Token<'t>>,
    pub languages: Option<Vec<Token<'t>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn project() {
        let src = include_bytes!("../../../../../fixture/commands/project/project");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::Project(Box::new(Project {
                    project_name: token(b"qqq"),
                    details: None,
                })),
                Command::Project(Box::new(Project {
                    project_name: token(b"aaa"),
                    details: Some(ProjectDetails::Short(tokens_vec([b"C", b"Rust"]))),
                })),
                Command::Project(Box::new(Project {
                    project_name: token(b"bbb"),
                    details: Some(ProjectDetails::General(GeneralProjectDetails {
                        version: Some(token(b"1.0.0")),
                        description: Some(token(b"Project bbb")),
                        homepage_url: Some(token(b"https://qqq.qqq")),
                        languages: Some(tokens_vec([b"C", b"Rust"])),
                    })),
                })),
            ])
        )
    }
}
