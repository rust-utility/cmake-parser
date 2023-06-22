use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Manage CMake Policy settings.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/cmake_policy.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum CMakePolicy<'t> {
    Version(PolicyVersion<'t>),
    Set(PolicySet<'t>),
    Get(PolicyGet<'t>),
    Push,
    Pop,
}

impl<'t> ToCommandScope for CMakePolicy<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PolicyVersion<'t> {
    pub min: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PolicySet<'t> {
    pub policy: Token<'t>,
    pub behavior: Behavior,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum Behavior {
    New,
    Old,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PolicyGet<'t> {
    pub policy: Token<'t>,
    pub variable: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn cmake_policy() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/cmake_policy");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CMakePolicy(Box::new(CMakePolicy::Version(PolicyVersion {
                    min: token(b"min1"),
                }))),
                Command::CMakePolicy(Box::new(CMakePolicy::Set(PolicySet {
                    policy: token(b"CMP0001"),
                    behavior: Behavior::New,
                }))),
                Command::CMakePolicy(Box::new(CMakePolicy::Set(PolicySet {
                    policy: token(b"CMP0001"),
                    behavior: Behavior::Old,
                }))),
                Command::CMakePolicy(Box::new(CMakePolicy::Get(PolicyGet {
                    policy: token(b"CMP0001"),
                    variable: token(b"variable1"),
                }))),
                Command::CMakePolicy(Box::new(CMakePolicy::Push)),
                Command::CMakePolicy(Box::new(CMakePolicy::Pop)),
            ])
        )
    }
}
