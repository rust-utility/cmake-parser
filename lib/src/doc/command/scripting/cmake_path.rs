use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// This command is for the manipulation of paths.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/cmake_path.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum CMakePath<'t> {
    Decomposition(CMakePathDecomposition<'t>),
    Query(CMakePathQuery<'t>),
    Modification(CMakePathModification<'t>),
    Generation(CMakePathGeneration<'t>),
    NativeConversion(CMakePathNativeConversion<'t>),
    #[cmake(rename = "HASH", transparent)]
    Hashing(CMakePathHashing<'t>),
}

impl<'t> ToCommandScope for CMakePath<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CMakePathDecomposition<'t> {
    #[cmake(rename = "GET", transparent)]
    pub path_var: Token<'t>,
    pub component: PathComponent<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum PathComponent<'t> {
    RootName(Token<'t>),
    RootDirectory(Token<'t>),
    RootPath(Token<'t>),
    Filename(Token<'t>),
    Extension(PathComponentExtension<'t>),
    Stem(PathComponentStem<'t>),
    RelativePart(Token<'t>),
    ParentPath(Token<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PathComponentExtension<'t> {
    pub last_only: bool,
    pub out_var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PathComponentStem<'t> {
    pub last_only: bool,
    pub out_var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum CMakePathQuery<'t> {
    HasRootName(PathQuery<'t>),
    HasRootDirectory(PathQuery<'t>),
    HasRootPath(PathQuery<'t>),
    HasFilename(PathQuery<'t>),
    HasExtension(PathQuery<'t>),
    HasStem(PathQuery<'t>),
    HasRelativePart(PathQuery<'t>),
    HasParentPath(PathQuery<'t>),
    IsAbsolute(PathQuery<'t>),
    IsRelative(PathQuery<'t>),
    IsPrefix(PathQueryPrefix<'t>),
    Compare(PathQueryCompare<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PathQuery<'t> {
    pub path_var: Token<'t>,
    pub out_var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PathQueryPrefix<'t> {
    pub path_var: Token<'t>,
    pub input: Token<'t>,
    pub normalize: bool,
    pub out_var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PathQueryCompare<'t> {
    pub input1: Token<'t>,
    pub operation: CompareOperation,
    pub input2: Token<'t>,
    pub out_var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum CompareOperation {
    Equal,
    NotEqual,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum CMakePathModification<'t> {
    Set(PathModificationSet<'t>),
    Append(PathModificationAppend<'t>),
    AppendString(PathModificationAppendString<'t>),
    RemoveFilename(PathModificationRemoveFilename<'t>),
    ReplaceFilename(PathModificationReplaceFilename<'t>),
    RemoveExtension(PathModificationRemoveExtension<'t>),
    ReplaceExtension(PathModificationReplaceExtension<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PathModificationSet<'t> {
    pub path_var: Token<'t>,
    pub normalize: bool,
    pub input: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "inputs")]
pub struct PathModificationAppend<'t> {
    #[cmake(positional)]
    pub path_var: Token<'t>,
    #[cmake(rename = "")]
    pub inputs: Option<Vec<Token<'t>>>,
    #[cmake(rename = "OUTPUT_VARIABLE")]
    pub out_var: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "inputs")]
pub struct PathModificationAppendString<'t> {
    #[cmake(positional)]
    pub path_var: Token<'t>,
    #[cmake(rename = "")]
    pub inputs: Option<Vec<Token<'t>>>,
    #[cmake(rename = "OUTPUT_VARIABLE")]
    pub out_var: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PathModificationRemoveFilename<'t> {
    pub path_var: Token<'t>,
    #[cmake(rename = "OUTPUT_VARIABLE", transparent)]
    pub out_var: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PathModificationReplaceFilename<'t> {
    pub path_var: Token<'t>,
    pub input: Token<'t>,
    #[cmake(rename = "OUTPUT_VARIABLE", transparent)]
    pub out_var: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PathModificationRemoveExtension<'t> {
    pub path_var: Token<'t>,
    pub last_only: bool,
    #[cmake(rename = "OUTPUT_VARIABLE", transparent)]
    pub out_var: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PathModificationReplaceExtension<'t> {
    pub path_var: Token<'t>,
    pub last_only: bool,
    pub input: Token<'t>,
    #[cmake(rename = "OUTPUT_VARIABLE", transparent)]
    pub out_var: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum CMakePathGeneration<'t> {
    #[cmake(rename = "NORMAL_PATH")]
    Normal(PathGenerationNormal<'t>),
    #[cmake(rename = "RELATIVE_PATH")]
    Relative(PathGenerationRelative<'t>),
    #[cmake(rename = "ABSOLUTE_PATH")]
    Absolute(PathGenerationAbsolute<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct PathGenerationNormal<'t> {
    pub path_var: Token<'t>,
    #[cmake(rename = "OUTPUT_VARIABLE", transparent)]
    pub out_var: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct PathGenerationRelative<'t> {
    #[cmake(positional)]
    pub path_var: Token<'t>,
    #[cmake(rename = "BASE_DIRECTORY")]
    pub input: Option<Token<'t>>,
    #[cmake(rename = "OUTPUT_VARIABLE")]
    pub out_var: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct PathGenerationAbsolute<'t> {
    #[cmake(positional)]
    pub path_var: Token<'t>,
    #[cmake(rename = "BASE_DIRECTORY")]
    pub input: Option<Token<'t>>,
    pub normalize: bool,
    #[cmake(rename = "OUTPUT_VARIABLE")]
    pub out_var: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum CMakePathNativeConversion<'t> {
    NativePath(NativeConversionPath<'t>),
    Convert(NativeConversionConvert<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct NativeConversionPath<'t> {
    pub path_var: Token<'t>,
    pub normalize: bool,
    pub out_var: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct NativeConversionConvert<'t> {
    pub input: Token<'t>,
    pub to: ConvertToPathList,
    pub out_var: Token<'t>,
    pub normalize: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum ConvertToPathList {
    #[cmake(rename = "TO_CMAKE_PATH_LIST")]
    CMake,
    #[cmake(rename = "TO_NATIVE_PATH_LIST")]
    Native,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CMakePathHashing<'t> {
    pub path_var: Token<'t>,
    pub out_var: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn cmake_path() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/cmake_path");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                // Decomposition
                Command::CMakePath(Box::new(CMakePath::Decomposition(CMakePathDecomposition {
                    path_var: token(b"path_var1"),
                    component: PathComponent::RootName(token(b"out_var1")),
                }))),
                Command::CMakePath(Box::new(CMakePath::Decomposition(CMakePathDecomposition {
                    path_var: token(b"path_var1"),
                    component: PathComponent::RootDirectory(token(b"out_var1")),
                }))),
                Command::CMakePath(Box::new(CMakePath::Decomposition(CMakePathDecomposition {
                    path_var: token(b"path_var1"),
                    component: PathComponent::RootPath(token(b"out_var1")),
                }))),
                Command::CMakePath(Box::new(CMakePath::Decomposition(CMakePathDecomposition {
                    path_var: token(b"path_var1"),
                    component: PathComponent::Filename(token(b"out_var1")),
                }))),
                Command::CMakePath(Box::new(CMakePath::Decomposition(CMakePathDecomposition {
                    path_var: token(b"path_var1"),
                    component: PathComponent::Extension(PathComponentExtension {
                        last_only: true,
                        out_var: token(b"out_var1"),
                    }),
                }))),
                Command::CMakePath(Box::new(CMakePath::Decomposition(CMakePathDecomposition {
                    path_var: token(b"dotPath"),
                    component: PathComponent::Extension(PathComponentExtension {
                        last_only: false,
                        out_var: token(b"dotExt"),
                    }),
                }))),
                Command::CMakePath(Box::new(CMakePath::Decomposition(CMakePathDecomposition {
                    path_var: token(b"path_var1"),
                    component: PathComponent::Stem(PathComponentStem {
                        last_only: true,
                        out_var: token(b"out_var1"),
                    }),
                }))),
                Command::CMakePath(Box::new(CMakePath::Decomposition(CMakePathDecomposition {
                    path_var: token(b"dotPath"),
                    component: PathComponent::Stem(PathComponentStem {
                        last_only: false,
                        out_var: token(b"dotStem"),
                    }),
                }))),
                Command::CMakePath(Box::new(CMakePath::Decomposition(CMakePathDecomposition {
                    path_var: token(b"path_var1"),
                    component: PathComponent::RelativePart(token(b"out_var1")),
                }))),
                Command::CMakePath(Box::new(CMakePath::Decomposition(CMakePathDecomposition {
                    path_var: token(b"path_var1"),
                    component: PathComponent::ParentPath(token(b"out_var1")),
                }))),
                // Query
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::HasRootName(
                    PathQuery {
                        path_var: token(b"path_var1"),
                        out_var: token(b"out_var1"),
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(
                    CMakePathQuery::HasRootDirectory(PathQuery {
                        path_var: token(b"path_var1"),
                        out_var: token(b"out_var1"),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::HasRootPath(
                    PathQuery {
                        path_var: token(b"path_var1"),
                        out_var: token(b"out_var1"),
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::HasFilename(
                    PathQuery {
                        path_var: token(b"path_var1"),
                        out_var: token(b"out_var1"),
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::HasExtension(
                    PathQuery {
                        path_var: token(b"path_var1"),
                        out_var: token(b"out_var1"),
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::HasStem(
                    PathQuery {
                        path_var: token(b"path_var1"),
                        out_var: token(b"out_var1"),
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::HasRelativePart(
                    PathQuery {
                        path_var: token(b"path_var1"),
                        out_var: token(b"out_var1"),
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::HasParentPath(
                    PathQuery {
                        path_var: token(b"path_var1"),
                        out_var: token(b"out_var1"),
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::IsAbsolute(
                    PathQuery {
                        path_var: token(b"path_var1"),
                        out_var: token(b"out_var1"),
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::IsRelative(
                    PathQuery {
                        path_var: token(b"path_var1"),
                        out_var: token(b"out_var1"),
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::IsPrefix(
                    PathQueryPrefix {
                        path_var: token(b"path_var1"),
                        input: token(b"input1"),
                        normalize: false,
                        out_var: token(b"out_var1"),
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::IsPrefix(
                    PathQueryPrefix {
                        path_var: token(b"path_var1"),
                        input: token(b"input1"),
                        normalize: true,
                        out_var: token(b"out_var1"),
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::Compare(
                    PathQueryCompare {
                        input1: token(b"input1"),
                        operation: CompareOperation::Equal,
                        input2: token(b"input2"),
                        out_var: token(b"out_var1")
                    }
                )))),
                Command::CMakePath(Box::new(CMakePath::Query(CMakePathQuery::Compare(
                    PathQueryCompare {
                        input1: token(b"input1"),
                        operation: CompareOperation::NotEqual,
                        input2: token(b"input2"),
                        out_var: token(b"out_var1")
                    }
                )))),
                // Modification
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::Set(PathModificationSet {
                        path_var: token(b"path_var1"),
                        normalize: true,
                        input: token(b"input1"),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::Set(PathModificationSet {
                        path_var: token(b"path_var1"),
                        normalize: false,
                        input: token(b"input1"),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::Append(PathModificationAppend {
                        path_var: token(b"path_var1"),
                        inputs: None,
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::Append(PathModificationAppend {
                        path_var: token(b"path_var1"),
                        inputs: Some(tokens_vec([b"input1", b"input2", b"input3"])),
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::Append(PathModificationAppend {
                        path_var: token(b"path_var1"),
                        inputs: Some(tokens_vec([b"input1", b"input2", b"input3"])),
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::Append(PathModificationAppend {
                        path_var: token(b"path_var1"),
                        inputs: None,
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::AppendString(PathModificationAppendString {
                        path_var: token(b"path_var1"),
                        inputs: None,
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::AppendString(PathModificationAppendString {
                        path_var: token(b"path_var1"),
                        inputs: Some(tokens_vec([b"input1", b"input2", b"input3"])),
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::AppendString(PathModificationAppendString {
                        path_var: token(b"path_var1"),
                        inputs: Some(tokens_vec([b"input1", b"input2", b"input3"])),
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::AppendString(PathModificationAppendString {
                        path_var: token(b"path_var1"),
                        inputs: None,
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::RemoveFilename(PathModificationRemoveFilename {
                        path_var: token(b"path_var1"),
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::RemoveFilename(PathModificationRemoveFilename {
                        path_var: token(b"path_var1"),
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::ReplaceFilename(PathModificationReplaceFilename {
                        path_var: token(b"path_var1"),
                        input: token(b"input1"),
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::ReplaceFilename(PathModificationReplaceFilename {
                        path_var: token(b"path_var1"),
                        input: token(b"input1"),
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::RemoveExtension(PathModificationRemoveExtension {
                        path_var: token(b"path_var1"),
                        last_only: false,
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::RemoveExtension(PathModificationRemoveExtension {
                        path_var: token(b"path_var1"),
                        last_only: false,
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::RemoveExtension(PathModificationRemoveExtension {
                        path_var: token(b"path_var1"),
                        last_only: true,
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::RemoveExtension(PathModificationRemoveExtension {
                        path_var: token(b"path_var1"),
                        last_only: true,
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::ReplaceExtension(PathModificationReplaceExtension {
                        path_var: token(b"path_var1"),
                        last_only: false,
                        input: token(b"input1"),
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::ReplaceExtension(PathModificationReplaceExtension {
                        path_var: token(b"path_var1"),
                        last_only: false,
                        input: token(b"input1"),
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::ReplaceExtension(PathModificationReplaceExtension {
                        path_var: token(b"path_var1"),
                        last_only: true,
                        input: token(b"input1"),
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Modification(
                    CMakePathModification::ReplaceExtension(PathModificationReplaceExtension {
                        path_var: token(b"path_var1"),
                        last_only: true,
                        input: token(b"input1"),
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                // Generation
                Command::CMakePath(Box::new(CMakePath::Generation(
                    CMakePathGeneration::Normal(PathGenerationNormal {
                        path_var: token(b"path_var1"),
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Generation(
                    CMakePathGeneration::Normal(PathGenerationNormal {
                        path_var: token(b"path_var1"),
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Generation(
                    CMakePathGeneration::Relative(PathGenerationRelative {
                        path_var: token(b"path_var1"),
                        input: None,
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Generation(
                    CMakePathGeneration::Relative(PathGenerationRelative {
                        path_var: token(b"path_var1"),
                        input: Some(token(b"input1")),
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Generation(
                    CMakePathGeneration::Relative(PathGenerationRelative {
                        path_var: token(b"path_var1"),
                        input: None,
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Generation(
                    CMakePathGeneration::Relative(PathGenerationRelative {
                        path_var: token(b"path_var1"),
                        input: Some(token(b"input1")),
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Generation(
                    CMakePathGeneration::Absolute(PathGenerationAbsolute {
                        path_var: token(b"path_var1"),
                        input: None,
                        normalize: false,
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Generation(
                    CMakePathGeneration::Absolute(PathGenerationAbsolute {
                        path_var: token(b"path_var1"),
                        input: Some(token(b"input1")),
                        normalize: false,
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Generation(
                    CMakePathGeneration::Absolute(PathGenerationAbsolute {
                        path_var: token(b"path_var1"),
                        input: None,
                        normalize: false,
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Generation(
                    CMakePathGeneration::Absolute(PathGenerationAbsolute {
                        path_var: token(b"path_var1"),
                        input: Some(token(b"input1")),
                        normalize: false,
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::Generation(
                    CMakePathGeneration::Absolute(PathGenerationAbsolute {
                        path_var: token(b"path_var1"),
                        input: Some(token(b"input1")),
                        normalize: true,
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                // Native Conversion
                Command::CMakePath(Box::new(CMakePath::NativeConversion(
                    CMakePathNativeConversion::NativePath(NativeConversionPath {
                        path_var: token(b"path_var1"),
                        normalize: false,
                        out_var: None,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::NativeConversion(
                    CMakePathNativeConversion::NativePath(NativeConversionPath {
                        path_var: token(b"path_var1"),
                        normalize: true,
                        out_var: Some(token(b"out_var1")),
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::NativeConversion(
                    CMakePathNativeConversion::Convert(NativeConversionConvert {
                        input: token(b"input1"),
                        to: ConvertToPathList::CMake,
                        out_var: token(b"out_var1"),
                        normalize: false,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::NativeConversion(
                    CMakePathNativeConversion::Convert(NativeConversionConvert {
                        input: token(b"input1"),
                        to: ConvertToPathList::CMake,
                        out_var: token(b"out_var1"),
                        normalize: true,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::NativeConversion(
                    CMakePathNativeConversion::Convert(NativeConversionConvert {
                        input: token(b"input1"),
                        to: ConvertToPathList::Native,
                        out_var: token(b"out_var1"),
                        normalize: false,
                    })
                ))),
                Command::CMakePath(Box::new(CMakePath::NativeConversion(
                    CMakePathNativeConversion::Convert(NativeConversionConvert {
                        input: token(b"input1"),
                        to: ConvertToPathList::Native,
                        out_var: token(b"out_var1"),
                        normalize: true,
                    })
                ))),
                // Hashing
                Command::CMakePath(Box::new(CMakePath::Hashing(CMakePathHashing {
                    path_var: token(b"path_var1"),
                    out_var: token(b"out_var1"),
                }))),
            ])
        )
    }
}
