use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Keyword, Token,
};

/// Add a library to the project using the specified source files.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/add_library.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct AddLibrary<'t> {
    pub name: Token<'t>,
    pub library: Library<'t>,
}

impl<'t> ToCommandScope for AddLibrary<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum Library<'t> {
    #[cmake(transparent)]
    Object(ObjectLibrary<'t>),
    #[cmake(transparent)]
    Interface(InterfaceLibrary<'t>),
    Imported(ImportedLibrary),
    #[cmake(transparent)]
    Alias(AliasLibrary<'t>),
    Normal(NormalLibrary<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ObjectLibrary<'t> {
    pub sources: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct InterfaceLibrary<'t> {
    pub sources: Option<Vec<Token<'t>>>,
    pub exclude_from_all: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "sources")]
pub struct NormalLibrary<'t> {
    pub library_type: Option<NormalLibraryType>,
    pub exclude_from_all: bool,
    #[cmake(rename = "")]
    pub sources: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum NormalLibraryType {
    Static,
    Shared,
    Module,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ImportedLibrary {
    pub library_type: ImportedLibraryType,
    imported: Keyword,
    pub global: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum ImportedLibraryType {
    Static,
    Shared,
    Module,
    Unknown,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct AliasLibrary<'t> {
    pub target: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn add_library() {
        let src = include_bytes!("../../../../../fixture/commands/project/add_library");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::AddLibrary(Box::new(AddLibrary {
                    name: b"MyProgram".into(),
                    library: Library::Normal(NormalLibrary {
                        library_type: Some(NormalLibraryType::Static),
                        exclude_from_all: true,
                        sources: Some(vec![b"my_program.cpp".into()])
                    })
                })),
                Command::AddLibrary(Box::new(AddLibrary {
                    name: b"ClangFormat".into(),
                    library: Library::Imported(ImportedLibrary {
                        library_type: ImportedLibraryType::Unknown,
                        imported: Keyword,
                        global: true
                    })
                })),
                Command::AddLibrary(Box::new(AddLibrary {
                    name: b"MyAliasedProgram".into(),
                    library: Library::Alias(AliasLibrary {
                        target: b"MyProgram".into()
                    })
                })),
                Command::AddLibrary(Box::new(AddLibrary {
                    name: b"MyInterfaceLib".into(),
                    library: Library::Interface(InterfaceLibrary {
                        sources: None,
                        exclude_from_all: false,
                    }),
                })),
                Command::AddLibrary(Box::new(AddLibrary {
                    name: b"ObjLib".into(),
                    library: Library::Object(ObjectLibrary {
                        sources: Some(vec![b"src1.c".into(), b"src2.c".into()])
                    })
                })),
                Command::AddLibrary(Box::new(AddLibrary {
                    name: b"kernels".into(),
                    library: Library::Normal(NormalLibrary {
                        library_type: None,
                        exclude_from_all: false,
                        sources: Some(vec![b"test.cu".into(), b"test.cuh".into()])
                    })
                })),
            ])
        );
    }
}
