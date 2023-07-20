use cmake_parser_derive::CMake;

use crate::{
    command::common::{FindPath as CommonFindPath, FindRoot, Names, WindowsRegistryView},
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// This command is used to find a program.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/find_program.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum FindProgram<'t> {
    General(FindProgramGeneral<'t>),
    Short(FindProgramShort<'t>),
}

impl<'t> ToCommandScope for FindProgram<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "names")]
pub struct FindProgramGeneral<'t> {
    #[cmake(positional)]
    pub variable: Token<'t>,
    #[cmake(rename = "")]
    pub names: Names<'t>,
    pub names_per_dir: bool,
    pub hints: Option<Vec<CommonFindPath<'t>>>,
    pub paths: Option<Vec<CommonFindPath<'t>>>,
    pub registry_view: Option<WindowsRegistryView>,
    pub path_suffixes: Option<Vec<Token<'t>>>,
    pub validator: Option<Token<'t>>,
    pub doc: Option<Token<'t>>,
    pub no_cache: bool,
    pub required: bool,
    pub no_default_path: bool,
    pub no_package_root_path: bool,
    pub no_cmake_path: bool,
    pub no_cmake_environment_path: bool,
    pub no_system_environment_path: bool,
    pub no_cmake_system_path: bool,
    pub no_cmake_install_prefix: bool,
    pub find_root: Option<FindRoot>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct FindProgramShort<'t> {
    pub variable: Token<'t>,
    pub name: Token<'t>,
    pub paths: Vec<Token<'t>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn find_program() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/find_program");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::FindProgram(Box::new(FindProgram::General(FindProgramGeneral {
                    variable: token(b"variable1"),
                    names: Names::Multi(tokens_vec([b"name1", b"name2"])),
                    names_per_dir: false,
                    hints: None,
                    paths: None,
                    registry_view: None,
                    path_suffixes: None,
                    validator: None,
                    doc: None,
                    no_cache: false,
                    required: false,
                    no_default_path: false,
                    no_package_root_path: false,
                    no_cmake_path: false,
                    no_cmake_environment_path: false,
                    no_system_environment_path: false,
                    no_cmake_system_path: false,
                    no_cmake_install_prefix: false,
                    find_root: None,
                }))),
                Command::FindProgram(Box::new(FindProgram::Short(FindProgramShort {
                    variable: token(b"variable1"),
                    name: token(b"name1"),
                    paths: tokens_vec([b"path1"]),
                }))),
                Command::FindProgram(Box::new(FindProgram::General(FindProgramGeneral {
                    variable: token(b"variable1"),
                    names: Names::Multi(tokens_vec([b"name1", b"name2"])),
                    names_per_dir: true,
                    hints: Some(vec![
                        CommonFindPath::Path(token(b"path1")),
                        CommonFindPath::Path(token(b"path2")),
                        CommonFindPath::Env(token(b"env1")),
                    ]),
                    paths: Some(vec![
                        CommonFindPath::Env(token(b"env1")),
                        CommonFindPath::Env(token(b"env2")),
                        CommonFindPath::Path(token(b"path1")),
                    ]),
                    registry_view: Some(WindowsRegistryView::Target),
                    path_suffixes: Some(tokens_vec([b"suffix1", b"suffix2"])),
                    validator: Some(token(b"validator1")),
                    doc: Some(token(b"doc1")),
                    no_cache: true,
                    required: true,
                    no_default_path: true,
                    no_package_root_path: true,
                    no_cmake_path: true,
                    no_cmake_environment_path: true,
                    no_system_environment_path: true,
                    no_cmake_system_path: true,
                    no_cmake_install_prefix: true,
                    find_root: Some(FindRoot::CMakeFindRootPathBoth),
                }))),
            ])
        )
    }
}
