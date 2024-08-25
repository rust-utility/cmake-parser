use cmake_parser_derive::CMake;

use crate::doc::command_scope::{CommandScope, ToCommandScope};

/// Provides an include guard for the file currently being processed by CMake.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/include_guard.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct IncludeGuard {
    pub scope: Option<Scope>,
}

impl ToCommandScope for IncludeGuard {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum Scope {
    Directory,
    Global,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn include_guard() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/include_guard");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::IncludeGuard(Box::new(IncludeGuard {
                    scope: None,
                }))),
                Ok(Command::IncludeGuard(Box::new(IncludeGuard {
                    scope: Some(Scope::Directory),
                }))),
                Ok(Command::IncludeGuard(Box::new(IncludeGuard {
                    scope: Some(Scope::Global),
                }))),
            ]
        )
    }
}
