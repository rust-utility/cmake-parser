use cmake_parser_derive::CMake;

use crate::{
    command::common::{FindRoot, WindowsRegistryView},
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Find a package
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/find_package.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum FindPackage<'t> {
    Full(FindPackageFull<'t>),
    Basic(FindPackageBasic<'t>),
}

impl<'t> ToCommandScope for FindPackage<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", complete, default = "version")]
pub struct FindPackageBasic<'t> {
    #[cmake(positional)]
    pub package_name: Token<'t>,
    #[cmake(rename = "")]
    pub version: Option<Token<'t>>,
    pub exact: bool,
    pub quiet: bool,
    pub module: bool,
    pub components: Option<PackageComponents<'t>>,
    pub optional_components: Option<Vec<Token<'t>>>,
    pub registry_view: Option<WindowsRegistryView>,
    pub global: bool,
    pub no_policy_scope: bool,
    pub bypass_provider: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", complete, default = "version", except = ["MODULE"])]
pub struct FindPackageFull<'t> {
    #[cmake(positional)]
    pub package_name: Token<'t>,
    #[cmake(rename = "")]
    pub version: Option<Token<'t>>,
    pub exact: bool,
    pub quiet: bool,
    pub components: Option<PackageComponents<'t>>,
    pub optional_components: Option<Vec<Token<'t>>>,
    pub config_mode: Option<ConfigMode>,
    pub global: bool,
    pub no_policy_scope: bool,
    pub bypass_provider: bool,
    pub names: Option<Vec<Token<'t>>>,
    pub configs: Option<Vec<Token<'t>>>,
    pub hints: Option<Vec<Token<'t>>>,
    pub paths: Option<Vec<Token<'t>>>,
    pub registry_view: Option<WindowsRegistryView>,
    pub path_suffixes: Option<Vec<Token<'t>>>,
    pub no_default_path: bool,
    pub no_package_root_path: bool,
    pub no_cmake_path: bool,
    pub no_cmake_environment_path: bool,
    pub no_system_environment_path: bool,
    pub no_cmake_package_registry: bool,
    pub no_cmake_builds_path: bool,
    pub no_cmake_system_path: bool,
    pub no_cmake_install_prefix: bool,
    pub find_root: Option<FindRoot>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional, complete)]
pub enum PackageComponents<'t> {
    Components(Components<'t>),
    #[cmake(transparent)]
    Required(Option<Components<'t>>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "components")]
pub struct Components<'t> {
    pub components: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum ConfigMode {
    Config,
    NoModule,
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn package_components() {
        #[derive(CMake, Debug, PartialEq, Eq)]
        #[cmake(pkg = "crate", allow_empty)]
        struct Test<'t> {
            components: Option<PackageComponents<'t>>,
        }
        assert_eq!(
            CMakeParse::complete(&tokens_vec([])),
            Ok(Test { components: None })
        );
        assert_eq!(
            CMakeParse::complete(&tokens_vec([b"COMPONENTS", b"component1"])),
            Ok(Test {
                components: Some(PackageComponents::Components(Components {
                    components: tokens_vec([b"component1"]),
                })),
            })
        );
        assert_eq!(
            CMakeParse::complete(&tokens_vec([b"REQUIRED"])),
            Ok(Test {
                components: Some(PackageComponents::Required(None)),
            })
        );
        assert_eq!(
            CMakeParse::complete(&tokens_vec([b"REQUIRED", b"COMPONENTS", b"component1"])),
            Ok(Test {
                components: Some(PackageComponents::Required(Some(Components {
                    components: tokens_vec([b"component1"]),
                }))),
            })
        );
        assert_eq!(
            CMakeParse::complete(&tokens_vec([b"REQUIRED", b"component1", b"component2"])),
            Ok(Test {
                components: Some(PackageComponents::Required(Some(Components {
                    components: tokens_vec([b"component1", b"component2"]),
                }))),
            })
        );
    }

    #[test]
    fn find_package() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/find_package");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::FindPackage(Box::new(FindPackage::Full(FindPackageFull {
                    package_name: token(b"package_name1"),
                    version: None,
                    exact: false,
                    quiet: false,
                    components: None,
                    optional_components: None,
                    config_mode: None,
                    global: false,
                    no_policy_scope: false,
                    bypass_provider: false,
                    names: None,
                    configs: None,
                    hints: None,
                    paths: None,
                    registry_view: None,
                    path_suffixes: None,
                    no_default_path: false,
                    no_package_root_path: false,
                    no_cmake_path: false,
                    no_cmake_environment_path: false,
                    no_system_environment_path: false,
                    no_cmake_package_registry: false,
                    no_cmake_builds_path: false,
                    no_cmake_system_path: false,
                    no_cmake_install_prefix: false,
                    find_root: None,
                }))),
                Command::FindPackage(Box::new(FindPackage::Basic(FindPackageBasic {
                    package_name: token(b"package_name1"),
                    version: None,
                    exact: false,
                    quiet: false,
                    module: true,
                    components: None,
                    optional_components: None,
                    global: false,
                    no_policy_scope: false,
                    bypass_provider: false,
                    registry_view: None,
                }))),
                Command::FindPackage(Box::new(FindPackage::Full(FindPackageFull {
                    package_name: token(b"package_name1"),
                    version: None,
                    exact: false,
                    quiet: false,
                    components: Some(PackageComponents::Required(Some(Components {
                        components: tokens_vec([b"component1", b"component2"]),
                    }))),
                    optional_components: None,
                    config_mode: None,
                    global: false,
                    no_policy_scope: false,
                    bypass_provider: false,
                    names: None,
                    configs: None,
                    hints: None,
                    paths: None,
                    registry_view: None,
                    path_suffixes: None,
                    no_default_path: false,
                    no_package_root_path: false,
                    no_cmake_path: false,
                    no_cmake_environment_path: false,
                    no_system_environment_path: false,
                    no_cmake_package_registry: false,
                    no_cmake_builds_path: false,
                    no_cmake_system_path: false,
                    no_cmake_install_prefix: false,
                    find_root: None,
                }))),
                Command::FindPackage(Box::new(FindPackage::Basic(FindPackageBasic {
                    package_name: token(b"package_name1"),
                    version: None,
                    exact: false,
                    quiet: false,
                    module: true,
                    components: Some(PackageComponents::Required(Some(Components {
                        components: tokens_vec([b"component1", b"component2"]),
                    }))),
                    optional_components: None,
                    global: false,
                    no_policy_scope: false,
                    bypass_provider: false,
                    registry_view: None,
                }))),
                Command::FindPackage(Box::new(FindPackage::Basic(FindPackageBasic {
                    package_name: token(b"package_name1"),
                    version: None,
                    exact: false,
                    quiet: false,
                    module: true,
                    components: Some(PackageComponents::Components(Components {
                        components: tokens_vec([b"component1", b"component2"]),
                    })),
                    optional_components: None,
                    global: false,
                    no_policy_scope: false,
                    bypass_provider: false,
                    registry_view: None,
                }))),
                Command::FindPackage(Box::new(FindPackage::Basic(FindPackageBasic {
                    package_name: token(b"package_name1"),
                    version: None,
                    exact: false,
                    quiet: false,
                    module: true,
                    components: Some(PackageComponents::Required(None)),
                    optional_components: None,
                    global: false,
                    no_policy_scope: false,
                    bypass_provider: false,
                    registry_view: None,
                }))),
                Command::FindPackage(Box::new(FindPackage::Full(FindPackageFull {
                    package_name: token(b"package_name1"),
                    version: Some(token(b"version1")),
                    exact: true,
                    quiet: true,
                    components: Some(PackageComponents::Required(Some(Components {
                        components: tokens_vec([b"component1", b"component2"]),
                    }))),
                    optional_components: Some(tokens_vec([b"component1", b"component2"])),
                    config_mode: Some(ConfigMode::NoModule),
                    global: true,
                    no_policy_scope: true,
                    bypass_provider: true,
                    names: Some(tokens_vec([b"name1", b"name2"])),
                    configs: Some(tokens_vec([b"config1", b"config2"])),
                    hints: Some(tokens_vec([b"hint1", b"hint2"])),
                    paths: Some(tokens_vec([b"path1", b"path2"])),
                    registry_view: Some(WindowsRegistryView::Bits64Fallback32),
                    path_suffixes: Some(tokens_vec([b"suffix1", b"suffix2"])),
                    no_default_path: true,
                    no_package_root_path: true,
                    no_cmake_path: true,
                    no_cmake_environment_path: true,
                    no_system_environment_path: true,
                    no_cmake_package_registry: true,
                    no_cmake_builds_path: true,
                    no_cmake_system_path: true,
                    no_cmake_install_prefix: true,
                    find_root: Some(FindRoot::OnlyCMakeFindRootPath),
                }))),
                Command::FindPackage(Box::new(FindPackage::Basic(FindPackageBasic {
                    package_name: token(b"package_name1"),
                    version: Some(token(b"version1")),
                    exact: true,
                    quiet: true,
                    module: true,
                    components: Some(PackageComponents::Components(Components {
                        components: tokens_vec([b"component1", b"component2"]),
                    })),
                    optional_components: Some(tokens_vec([b"component1", b"component2"])),
                    registry_view: Some(WindowsRegistryView::Host),
                    global: true,
                    no_policy_scope: true,
                    bypass_provider: true,
                }))),
            ])
        )
    }
}
