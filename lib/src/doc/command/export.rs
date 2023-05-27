use ::cmake_parser_derive::CMake;

use crate::{CommandScope, ToCommandScope, Token};

/// Export targets or packages for outside projects to use them directly from the current project's build tree, without installation.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/export.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum Export<'t> {
    Targets(TargetsExport<'t>),
    Export(ExportExport<'t>),
    Package(PackageExport<'t>),
}

impl<'t> ToCommandScope for Export<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum TargetsExport<'t> {
    File(FileTargetsExport<'t>),
    AndroidMk(AndroidMkTargetsExport<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileTargetsExport<'t> {
    pub targets: Vec<Token<'t>>,
    pub namespace: Option<Token<'t>>,
    pub append: bool,
    pub file: Token<'t>,
    pub export_link_interface_libraries: bool,
    pub cxx_modules_directory: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct AndroidMkTargetsExport<'t> {
    pub targets: Vec<Token<'t>>,
    pub android_mk: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct ExportExport<'t> {
    #[cmake(rename = "EXPORT")]
    pub exports: Vec<Token<'t>>,
    pub namespace: Option<Token<'t>>,
    pub file: Option<Token<'t>>,
    pub cxx_modules_directory: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct PackageExport<'t> {
    pub package: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{quoted_token, token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn export() {
        let src = include_bytes!("../../../../fixture/commands/project/export");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);

        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::Export(Box::new(Export::Targets(TargetsExport::File(
                    FileTargetsExport {
                        targets: tokens_vec([b"MathFunctionsTargets"]),
                        namespace: None,
                        append: false,
                        file: quoted_token(
                            b"${CMAKE_CURRENT_BINARY_DIR}/MathFunctionsTargets.cmake",
                        ),
                        export_link_interface_libraries: false,
                        cxx_modules_directory: None,
                    }
                )))),
                Command::Export(Box::new(Export::Export(ExportExport {
                    exports: tokens_vec([b"MathFunctionsTargets"]),
                    namespace: None,
                    file: Some(quoted_token(
                        b"${CMAKE_CURRENT_BINARY_DIR}/MathFunctionsTargets.cmake",
                    )),
                    cxx_modules_directory: None,
                }))),
                Command::Export(Box::new(Export::Targets(TargetsExport::AndroidMk(
                    AndroidMkTargetsExport {
                        targets: tokens_vec([b"android1", b"android2"]),
                        android_mk: token(b"../NDK1"),
                    }
                )))),
                Command::Export(Box::new(Export::Package(PackageExport {
                    package: token(b"hello"),
                }))),
            ])
        )
    }
}
