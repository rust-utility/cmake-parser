use cmake_parser_derive::CMake;

use crate::{
    command::common::{FileMatch, Permission},
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Specify rules to run at install time.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/install.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum Install<'t> {
    Targets(TargetsInstall<'t>),
    ImportedRuntimeArtifacts(ImportedRuntimeArtifactsInstall<'t>),
    #[cmake(transparent)]
    Files(FilesInstall<'t>),
    #[cmake(transparent)]
    Programs(FilesInstall<'t>),
    Directory(DirectoryInstall<'t>),
    Script(ScriptInstall<'t>),
    Code(ScriptInstall<'t>),
    #[cmake(transparent)]
    Export(ExportInstall<'t>),
    #[cmake(transparent)]
    ExportAndroidMk(ExportInstall<'t>),
    RuntimeDependencySet(RuntimeDependencySetInstall<'t>),
}

impl<'t> ToCommandScope for Install<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct TargetsInstall<'t> {
    pub targets: Vec<Token<'t>>,
    pub export: Option<Token<'t>>,
    pub runtime_dependency: Option<RuntimeDependency<'t>>,
    pub output_artifacts: Option<Vec<OutputArtifactTargets<'t>>>,
    pub includes: Option<IncludesDestination<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct ImportedRuntimeArtifactsInstall<'t> {
    pub imported_runtime_artifacts: Vec<Token<'t>>,
    pub runtime_dependency_set: Option<RuntimeDependencySet<'t>>,
    pub output_artifacts: Option<Vec<OutputArtifactImportedRuntimeArtifacts<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "files")]
pub struct FilesInstall<'t> {
    #[cmake(rename = "")]
    pub files: Vec<Token<'t>>,
    pub kind: InstallKind<'t>,
    pub permissions: Option<Vec<Permission>>,
    pub configurations: Option<Vec<Token<'t>>>,
    pub component: Option<Token<'t>>,
    pub rename: Option<Token<'t>>,
    pub optional: bool,
    pub exclude_from_all: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct DirectoryInstall<'t> {
    #[cmake(rename = "DIRECTORY")]
    pub dirs: Vec<Token<'t>>,
    pub kind: InstallKind<'t>,
    pub file_permissions: Option<Vec<Permission>>,
    pub directory_permissions: Option<Vec<Permission>>,
    pub use_source_permissions: bool,
    pub optional: bool,
    pub message_never: bool,
    pub configurations: Option<Vec<Token<'t>>>,
    pub component: Option<Token<'t>>,
    pub files_matching: bool,
    pub file_matches: Option<Vec<FileMatch<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "scripts")]
pub struct ScriptInstall<'t> {
    #[cmake(rename = "")]
    pub scripts: Vec<ScriptKind<'t>>,
    pub component: Option<ScriptComponent<'t>>,
    pub exclude_from_all: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct ExportInstall<'t> {
    #[cmake(positional)]
    pub export_name: Token<'t>,
    pub destination: Token<'t>,
    pub namespace: Option<Token<'t>>,
    pub file: Option<Token<'t>>,
    pub permissions: Option<Vec<Permission>>,
    pub configurations: Option<Vec<Token<'t>>>,
    pub cxx_modules_directory: Option<Token<'t>>,
    pub export_link_interface_libraries: bool,
    pub component: Option<Token<'t>>,
    pub exclude_from_all: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct RuntimeDependencySetInstall<'t> {
    #[cmake(rename = "RUNTIME_DEPENDENCY_SET")]
    pub set_name: Token<'t>,
    pub runtime_dependency_sets: Option<Vec<Dependency<'t>>>,
    pub pre_include_regexes: Option<Vec<Token<'t>>>,
    pub pre_exclude_regexes: Option<Vec<Token<'t>>>,
    pub post_include_regexes: Option<Vec<Token<'t>>>,
    pub post_exclude_regexes: Option<Vec<Token<'t>>>,
    pub post_include_files: Option<Vec<Token<'t>>>,
    pub post_exclude_files: Option<Vec<Token<'t>>>,
    pub directories: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", match_fields)]
pub struct Dependency<'t> {
    pub kind: Option<DependencyKind>,
    pub destination: Option<Token<'t>>,
    pub permissions: Option<Vec<Permission>>,
    pub configurations: Option<Vec<Token<'t>>>,
    pub component: Option<Token<'t>>,
    pub namelink_component: Option<Token<'t>>,
    pub optional: bool,
    pub exclude_from_all: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum DependencyKind {
    Library,
    Runtime,
    Framework,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum ScriptKind<'t> {
    Script(Token<'t>),
    Code(Token<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum ScriptComponent<'t> {
    AllComponents,
    Component(Token<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum RuntimeDependency<'t> {
    RuntimeDependencies(RuntimeDependencies<'t>),
    RuntimeDependencySet(RuntimeDependencySet<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum InstallKind<'t> {
    Type(InstallKindType),
    Destination(Token<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum InstallKindType {
    Bin,
    Sbin,
    Lib,
    Include,
    #[cmake(rename = "SYSCONF")]
    SysConf,
    #[cmake(rename = "SHAREDSTATE")]
    SharedState,
    #[cmake(rename = "LOCALSTATE")]
    LocalState,
    #[cmake(rename = "RUNSTATE")]
    RunState,
    Data,
    Info,
    Locale,
    Man,
    Doc,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct RuntimeDependencies<'t> {
    pub deps: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct RuntimeDependencySet<'t> {
    pub set_name: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", match_fields)]
pub struct OutputArtifactTargets<'t> {
    pub kind: Option<ArtifactKindTargets<'t>>,
    pub destination: Option<Token<'t>>,
    pub permissions: Option<Vec<Permission>>,
    pub configurations: Option<Vec<Token<'t>>>,
    pub component: Option<Token<'t>>,
    pub namelink_component: Option<Token<'t>>,
    pub optional: bool,
    pub exclude_from_all: bool,
    pub namelink: Option<Namelink>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", match_fields)]
pub struct OutputArtifactImportedRuntimeArtifacts<'t> {
    pub kind: Option<ArtifactKindImportedRuntimeArtifacts>,
    pub destination: Option<Token<'t>>,
    pub permissions: Option<Vec<Permission>>,
    pub configurations: Option<Vec<Token<'t>>>,
    pub component: Option<Token<'t>>,
    pub optional: bool,
    pub exclude_from_all: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum ArtifactKindTargets<'t> {
    Archive,
    Library,
    Runtime,
    Objects,
    Framework,
    Bundle,
    PrivateHeader,
    PublicHeader,
    Resource,
    #[cmake(transparent)]
    FileSet(Token<'t>),
    CssModulesBmi,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum ArtifactKindImportedRuntimeArtifacts {
    Library,
    Runtime,
    Framework,
    Bundle,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum Namelink {
    NamelinkOnly,
    NamelinkSkip,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IncludesDestination<'t> {
    dirs: Option<Vec<Token<'t>>>,
}

impl<'t> crate::CMakeParse<'t> for IncludesDestination<'t> {
    fn matches_type(_: &[u8], keyword: &[u8], tokens: &[Token<'t>]) -> bool {
        keyword == b"INCLUDES" && tokens.first().map(|x| x.as_bytes()) == Some(b"DESTINATION")
    }

    fn need_push_keyword(_: &Token<'t>) -> bool {
        false
    }

    fn rest<'tv>(tokens: &'tv [Token<'t>]) -> &'tv [Token<'t>] {
        &tokens[1..]
    }

    fn parse<'tv>(
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), crate::CommandParseError> {
        crate::CMakeParse::parse(tokens).map(|(dirs, tokens)| (Self { dirs }, tokens))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::common::FileMatchKind;
    use crate::doc::cmake_parse::tests::{quoted_token, quoted_tokens_vec, token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn install() {
        let src = include_bytes!("../../../../../fixture/commands/project/install");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::Install(Box::new(Install::Files(FilesInstall {
                    files: tokens_vec([b"${LIBXML2_HDRS}"]),
                    kind: InstallKind::Destination(token(
                        b"${CMAKE_INSTALL_INCLUDEDIR}/libxml2/libxml"
                    )),
                    permissions: None,
                    configurations: None,
                    component: Some(token(b"development")),
                    rename: None,
                    optional: false,
                    exclude_from_all: false,
                }))),
                Command::Install(Box::new(Install::Targets(TargetsInstall {
                    targets: tokens_vec([b"LibXml2"]),
                    export: Some(token(b"LibXml2")),
                    runtime_dependency: None,
                    output_artifacts: Some(vec![
                        OutputArtifactTargets {
                            kind: Some(ArtifactKindTargets::Archive),
                            destination: Some(token(b"${CMAKE_INSTALL_LIBDIR}")),
                            permissions: None,
                            configurations: None,
                            component: Some(token(b"development")),
                            namelink_component: None,
                            optional: false,
                            exclude_from_all: false,
                            namelink: None
                        },
                        OutputArtifactTargets {
                            kind: Some(ArtifactKindTargets::Library),
                            destination: Some(token(b"${CMAKE_INSTALL_LIBDIR}")),
                            permissions: None,
                            configurations: None,
                            component: Some(token(b"runtime")),
                            namelink_component: Some(token(b"development")),
                            optional: false,
                            exclude_from_all: false,
                            namelink: None
                        },
                        OutputArtifactTargets {
                            kind: Some(ArtifactKindTargets::Runtime),
                            destination: Some(token(b"${CMAKE_INSTALL_BINDIR}")),
                            permissions: None,
                            configurations: None,
                            component: Some(token(b"runtime")),
                            namelink_component: None,
                            optional: false,
                            exclude_from_all: false,
                            namelink: None
                        },
                    ]),
                    includes: Some(IncludesDestination {
                        dirs: Some(tokens_vec([b"qqq"]))
                    }),
                }))),
                Command::Install(Box::new(Install::Programs(FilesInstall {
                    files: tokens_vec([b"$<TARGET_PDB_FILE:LibXml2>"]),
                    kind: InstallKind::Type(InstallKindType::Sbin),
                    permissions: None,
                    configurations: Some(tokens_vec([b"Debug", b"RelWithDebInfo"])),
                    component: Some(token(b"debug")),
                    rename: None,
                    optional: false,
                    exclude_from_all: false,
                }))),
                Command::Install(Box::new(Install::Targets(TargetsInstall {
                    targets: tokens_vec([b"mylib"]),
                    export: None,
                    runtime_dependency: None,
                    output_artifacts: Some(vec![OutputArtifactTargets {
                        kind: Some(ArtifactKindTargets::PublicHeader),
                        destination: Some(token(b"${CMAKE_INSTALL_INCLUDEDIR}/myproj")),
                        permissions: None,
                        configurations: None,
                        component: None,
                        namelink_component: None,
                        optional: false,
                        exclude_from_all: false,
                        namelink: None
                    },]),
                    includes: None,
                }))),
                Command::Install(Box::new(Install::Targets(TargetsInstall {
                    targets: tokens_vec([b"mylib"]),
                    export: None,
                    runtime_dependency: None,
                    output_artifacts: Some(vec![
                        OutputArtifactTargets {
                            kind: Some(ArtifactKindTargets::Library),
                            destination: None,
                            permissions: None,
                            configurations: None,
                            component: Some(token(b"Libraries")),
                            namelink_component: Some(token(b"Development")),
                            optional: false,
                            exclude_from_all: false,
                            namelink: None
                        },
                        OutputArtifactTargets {
                            kind: Some(ArtifactKindTargets::PublicHeader),
                            destination: None,
                            permissions: None,
                            configurations: None,
                            component: Some(token(b"Development")),
                            namelink_component: None,
                            optional: false,
                            exclude_from_all: false,
                            namelink: None
                        },
                    ]),
                    includes: None,
                }))),
                Command::Install(Box::new(Install::Targets(TargetsInstall {
                    targets: tokens_vec([b"myExe", b"mySharedLib", b"myStaticLib"]),
                    export: None,
                    runtime_dependency: None,
                    output_artifacts: Some(vec![
                        OutputArtifactTargets {
                            kind: Some(ArtifactKindTargets::Runtime),
                            destination: Some(token(b"bin")),
                            permissions: None,
                            configurations: None,
                            component: None,
                            namelink_component: None,
                            optional: false,
                            exclude_from_all: false,
                            namelink: None
                        },
                        OutputArtifactTargets {
                            kind: Some(ArtifactKindTargets::Library),
                            destination: Some(token(b"lib")),
                            permissions: None,
                            configurations: None,
                            component: None,
                            namelink_component: None,
                            optional: false,
                            exclude_from_all: false,
                            namelink: None
                        },
                        OutputArtifactTargets {
                            kind: Some(ArtifactKindTargets::Archive),
                            destination: Some(token(b"lib/static")),
                            permissions: None,
                            configurations: None,
                            component: None,
                            namelink_component: None,
                            optional: false,
                            exclude_from_all: false,
                            namelink: None
                        },
                    ]),
                    includes: None,
                }))),
                Command::Install(Box::new(Install::Targets(TargetsInstall {
                    targets: tokens_vec([b"mySharedLib"]),
                    export: None,
                    runtime_dependency: None,
                    output_artifacts: Some(vec![OutputArtifactTargets {
                        kind: None,
                        destination: Some(token(b"/some/full/path")),
                        permissions: None,
                        configurations: None,
                        component: None,
                        namelink_component: None,
                        optional: false,
                        exclude_from_all: false,
                        namelink: None
                    },]),
                    includes: None,
                }))),
                Command::Install(Box::new(Install::ImportedRuntimeArtifacts(
                    ImportedRuntimeArtifactsInstall {
                        imported_runtime_artifacts: tokens_vec([b"${TBB_IMPORTED_TARGETS}"]),
                        runtime_dependency_set: Some(RuntimeDependencySet {
                            set_name: token(b"set-name"),
                        },),
                        output_artifacts: Some(vec![
                            OutputArtifactImportedRuntimeArtifacts {
                                kind: Some(ArtifactKindImportedRuntimeArtifacts::Library),
                                destination: Some(token(b"${CMAKE_INSTALL_LIBDIR}")),
                                permissions: None,
                                configurations: None,
                                component: None,
                                optional: false,
                                exclude_from_all: false,
                            },
                            OutputArtifactImportedRuntimeArtifacts {
                                kind: Some(ArtifactKindImportedRuntimeArtifacts::Runtime),
                                destination: Some(token(b"${CMAKE_INSTALL_BINDIR}")),
                                permissions: None,
                                configurations: None,
                                component: None,
                                optional: false,
                                exclude_from_all: false,
                            },
                        ]),
                    }
                ))),
                Command::Install(Box::new(Install::Directory(DirectoryInstall {
                    dirs: tokens_vec([b"src/"]),
                    kind: InstallKind::Destination(token(b"doc/myproj")),
                    file_permissions: None,
                    directory_permissions: None,
                    use_source_permissions: false,
                    optional: false,
                    message_never: false,
                    configurations: None,
                    component: None,
                    files_matching: true,
                    file_matches: Some(vec![FileMatch {
                        kind: Some(FileMatchKind::Pattern(quoted_token(b"*.png"),),),
                        exclude: false,
                        permissions: None,
                    },],),
                }))),
                Command::Install(Box::new(Install::Directory(DirectoryInstall {
                    dirs: tokens_vec([b"icons", b"scripts/"]),
                    kind: InstallKind::Destination(token(b"share/myproj")),
                    file_permissions: None,
                    directory_permissions: None,
                    use_source_permissions: false,
                    optional: false,
                    message_never: false,
                    configurations: None,
                    component: None,
                    files_matching: false,
                    file_matches: Some(vec![
                        FileMatch {
                            kind: Some(FileMatchKind::Pattern(quoted_token(b"CVS"),),),
                            exclude: true,
                            permissions: None,
                        },
                        FileMatch {
                            kind: Some(FileMatchKind::Pattern(quoted_token(b"scripts/*"),),),
                            exclude: false,
                            permissions: Some(vec![
                                Permission::OwnerExecute,
                                Permission::OwnerWrite,
                                Permission::OwnerRead,
                                Permission::GroupExecute,
                                Permission::GroupRead,
                            ]),
                        },
                    ],),
                }))),
                Command::Install(Box::new(Install::Code(ScriptInstall {
                    scripts: vec![
                        ScriptKind::Code(token(
                            b"message(STATUS \"HERE: ${CMAKE_INSTALL_PREFIX}\")"
                        )),
                        ScriptKind::Script(token(
                            b"message(STATUS \"HERE: ${CMAKE_INSTALL_PREFIX}\")"
                        )),
                    ],
                    component: Some(ScriptComponent::Component(token(b"qqq"))),
                    exclude_from_all: false,
                }))),
                Command::Install(Box::new(Install::Script(ScriptInstall {
                    scripts: vec![
                        ScriptKind::Script(token(
                            b"message(STATUS \"HERE: ${CMAKE_INSTALL_PREFIX}\")"
                        )),
                        ScriptKind::Code(token(
                            b"message(STATUS \"HERE: ${CMAKE_INSTALL_PREFIX}\")"
                        )),
                    ],
                    component: Some(ScriptComponent::AllComponents),
                    exclude_from_all: true,
                }))),
                Command::Install(Box::new(Install::Export(ExportInstall {
                    export_name: token(b"myproj"),
                    destination: token(b"lib/myproj"),
                    namespace: Some(token(b"mp_")),
                    file: None,
                    permissions: None,
                    configurations: None,
                    cxx_modules_directory: None,
                    export_link_interface_libraries: false,
                    component: None,
                    exclude_from_all: false
                }))),
                Command::Install(Box::new(Install::ExportAndroidMk(ExportInstall {
                    export_name: token(b"myproj"),
                    destination: token(b"share/ndk-modules"),
                    namespace: None,
                    file: None,
                    permissions: None,
                    configurations: None,
                    cxx_modules_directory: None,
                    export_link_interface_libraries: false,
                    component: None,
                    exclude_from_all: false
                }))),
                Command::Install(Box::new(Install::RuntimeDependencySet(
                    RuntimeDependencySetInstall {
                        set_name: token(b"myset"),
                        runtime_dependency_sets: None,
                        pre_include_regexes: Some(quoted_tokens_vec([b"dep[134]"])),
                        pre_exclude_regexes: Some(quoted_tokens_vec([b".*"])),
                        post_include_regexes: Some(quoted_tokens_vec([b"dep[13]"])),
                        post_exclude_regexes: Some(quoted_tokens_vec([b"dep[34]"])),
                        post_include_files: None,
                        post_exclude_files: None,
                        directories: Some(quoted_tokens_vec([b"$<TARGET_FILE_DIR:dep1>"])),
                    }
                ))),
                Command::Install(Box::new(Install::RuntimeDependencySet(
                    RuntimeDependencySetInstall {
                        set_name: token(b"myset"),
                        runtime_dependency_sets: Some(vec![
                            Dependency {
                                kind: Some(DependencyKind::Runtime),
                                destination: Some(token(b"yyy/bin")),
                                permissions: None,
                                configurations: None,
                                component: None,
                                namelink_component: None,
                                optional: false,
                                exclude_from_all: false,
                            },
                            Dependency {
                                kind: Some(DependencyKind::Library),
                                destination: Some(token(b"yyy/lib")),
                                permissions: None,
                                configurations: None,
                                component: None,
                                namelink_component: None,
                                optional: false,
                                exclude_from_all: false,
                            },
                        ]),
                        pre_include_regexes: Some(quoted_tokens_vec([b"dep[134]"])),
                        pre_exclude_regexes: Some(quoted_tokens_vec([b".*"])),
                        post_include_regexes: None,
                        post_exclude_regexes: None,
                        post_include_files: None,
                        post_exclude_files: None,
                        directories: Some(quoted_tokens_vec([b"$<TARGET_FILE_DIR:dep1>"])),
                    }
                ))),
                Command::Install(Box::new(Install::RuntimeDependencySet(
                    RuntimeDependencySetInstall {
                        set_name: token(b"myset"),
                        runtime_dependency_sets: Some(vec![Dependency {
                            kind: None,
                            destination: Some(token(b"zzz")),
                            permissions: None,
                            configurations: None,
                            component: None,
                            namelink_component: None,
                            optional: false,
                            exclude_from_all: false,
                        },]),
                        pre_include_regexes: Some(quoted_tokens_vec([b"dep[134]"])),
                        pre_exclude_regexes: Some(quoted_tokens_vec([b".*"])),
                        post_include_regexes: None,
                        post_exclude_regexes: None,
                        post_include_files: None,
                        post_exclude_files: None,
                        directories: Some(quoted_tokens_vec([b"$<TARGET_FILE_DIR:dep1>"])),
                    }
                ))),
            ])
        )
    }
}
