use cmake_parser_derive::CMake;

use crate::{
    command::common::{FindPath, FindRoot, Names, WindowsRegistryView},
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// This command is used to find a library.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/find_library.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum FindLibrary<'t> {
    General(FindLibraryGeneral<'t>),
    Short(FindLibraryShort<'t>),
}

impl<'t> ToCommandScope for FindLibrary<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "names")]
pub struct FindLibraryGeneral<'t> {
    #[cmake(positional)]
    pub variable: Token<'t>,
    #[cmake(rename = "")]
    pub names: Names<'t>,
    pub names_per_dir: bool,
    pub hints: Option<Vec<FindPath<'t>>>,
    pub paths: Option<Vec<FindPath<'t>>>,
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
pub struct FindLibraryShort<'t> {
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
    fn find_library() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/find_library");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::FindLibrary(Box::new(FindLibrary::General(FindLibraryGeneral {
                    variable: token(b"variable1"),
                    names: Names::Single(token(b"name1")),
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
                Command::FindLibrary(Box::new(FindLibrary::Short(FindLibraryShort {
                    variable: token(b"variable1"),
                    name: token(b"name1"),
                    paths: tokens_vec([b"path1"]),
                }))),
                Command::FindLibrary(Box::new(FindLibrary::General(FindLibraryGeneral {
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
                Command::FindLibrary(Box::new(FindLibrary::General(FindLibraryGeneral {
                    variable: token(b"variable1"),
                    names: Names::Multi(tokens_vec([b"name1", b"name2"])),
                    names_per_dir: true,
                    hints: Some(vec![
                        FindPath::Path(token(b"path1")),
                        FindPath::Path(token(b"path2")),
                        FindPath::Env(token(b"env1")),
                    ]),
                    paths: Some(vec![
                        FindPath::Env(token(b"env1")),
                        FindPath::Env(token(b"env2")),
                        FindPath::Path(token(b"path1")),
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
