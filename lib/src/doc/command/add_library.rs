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
    Object(ObjectLibrary<'t>),
    Interface,
    Imported(ImportedLibrary),
    Alias(AliasLibrary<'t>),
    Normal(NormalLibrary<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ObjectLibrary<'t> {
    object: Keyword,
    pub sources: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct NormalLibrary<'t> {
    pub library_type: NormalLibraryType,
    pub exclude_from_all: bool,
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
    alias: Keyword,
    pub target: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn add_compile_definitions() {
        let src = include_bytes!("../../../../fixture/commands/add_library");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::AddLibrary(Box::new(AddLibrary {
                    name: b"MyProgram".into(),
                    library: Library::Normal(NormalLibrary {
                        library_type: NormalLibraryType::Static,
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
                        alias: Keyword,
                        target: b"MyProgram".into()
                    })
                })),
                Command::AddLibrary(Box::new(AddLibrary {
                    name: b"MyInterfaceLib".into(),
                    library: Library::Interface,
                })),
                Command::AddLibrary(Box::new(AddLibrary {
                    name: b"ObjLib".into(),
                    library: Library::Object(ObjectLibrary {
                        object: Keyword,
                        sources: Some(vec![b"src1.c".into(), b"src2.c".into()])
                    })
                })),
            ])
        );
    }
}
