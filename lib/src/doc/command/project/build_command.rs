use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Get a command line to build the current project.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/build_command.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum BuildCommand<'t> {
    Variable(VariableBuildCommand<'t>),
    CacheVariable(CacheVariableBuildCommand<'t>),
}

impl<'t> ToCommandScope for BuildCommand<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct VariableBuildCommand<'t> {
    #[cmake(positional)]
    pub variable: Token<'t>,
    pub configuration: Option<Token<'t>>,
    pub parallel_level: Option<Token<'t>>,
    pub target: Option<Token<'t>>,
    pub project_name: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CacheVariableBuildCommand<'t> {
    pub cachevariable: Token<'t>,
    pub makecommand: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn build_command() {
        let src = include_bytes!("../../../../../fixture/commands/project/build_command");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::BuildCommand(Box::new(BuildCommand::Variable(VariableBuildCommand {
                    variable: b"var1".into(),
                    configuration: Some(b"cfg1".into()),
                    parallel_level: Some(b"plevel2".into()),
                    target: Some(b"target3".into()),
                    project_name: Some(b"project_name4".into())
                }))),
                Command::BuildCommand(Box::new(BuildCommand::CacheVariable(
                    CacheVariableBuildCommand {
                        cachevariable: b"cachevar1".into(),
                        makecommand: b"makecommand1".into(),
                    }
                ))),
            ])
        );
    }
}
