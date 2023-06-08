use cmake_parser_derive::CMake;

use crate::{
    command::common::{
        CopyFile, LangExtensions, LangStandard, LangStandardRequired, Source, SourceAlt,
    },
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Try building some code.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/try_compile.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum TryCompile<'t> {
    Sources(TryCompileProjectSources<'t>),
    SourcesAlt(TryCompileProjectSourcesAlt<'t>),
    Project(TryCompileProject<'t>),
    ProjectAlt(TryCompileProjectAlt<'t>),
}

impl<'t> ToCommandScope for TryCompile<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct TryCompileProject<'t> {
    #[cmake(positional)]
    pub compile_result_var: Token<'t>,
    #[cmake(positional, transparent)]
    pub project: Token<'t>,
    #[cmake(positional, transparent)]
    pub source_dir: Token<'t>,
    pub binary_dir: Option<Token<'t>>,
    pub target: Option<Token<'t>>,
    pub log_description: Option<Token<'t>>,
    pub no_cache: bool,
    pub no_log: bool,
    pub cmake_flags: Option<Vec<Token<'t>>>,
    pub output_variable: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "target")]
pub struct TryCompileProjectAlt<'t> {
    #[cmake(positional)]
    pub compile_result_var: Token<'t>,
    #[cmake(positional)]
    pub source_dir: Token<'t>,
    #[cmake(positional)]
    pub binary_dir: Token<'t>,
    #[cmake(positional)]
    pub project: Token<'t>,

    #[cmake(rename = "")]
    pub target: Option<Token<'t>>,
    pub cmake_flags: Option<Vec<Token<'t>>>,
    pub output_variable: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "sources")]
pub struct TryCompileProjectSources<'t> {
    #[cmake(positional)]
    pub compile_result_var: Token<'t>,

    #[cmake(rename = "")]
    pub sources: Vec<Source<'t>>,

    pub log_description: Option<Token<'t>>,
    pub no_cache: bool,
    pub no_log: bool,
    pub cmake_flags: Option<Vec<Token<'t>>>,
    pub compile_definitions: Option<Vec<Token<'t>>>,
    pub link_options: Option<Vec<Token<'t>>>,
    pub link_libraries: Option<Vec<Token<'t>>>,
    pub output_variable: Option<Token<'t>>,
    pub copy_file: Option<CopyFile<'t>>,
    pub lang_standard: Option<LangStandard<'t>>,
    pub lang_standard_required: Option<LangStandardRequired<'t>>,
    pub lang_extensions: Option<LangExtensions<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "sources")]
pub struct TryCompileProjectSourcesAlt<'t> {
    #[cmake(positional)]
    pub compile_result_var: Token<'t>,

    #[cmake(positional)]
    pub binary_dir: Token<'t>,

    #[cmake(rename = "")]
    pub sources: SourceAlt<'t>,

    pub cmake_flags: Option<Vec<Token<'t>>>,
    pub compile_definitions: Option<Vec<Token<'t>>>,
    pub link_options: Option<Vec<Token<'t>>>,
    pub link_libraries: Option<Vec<Token<'t>>>,
    pub output_variable: Option<Token<'t>>,
    pub copy_file: Option<CopyFile<'t>>,
    pub lang_standard: Option<LangStandard<'t>>,
    pub lang_standard_required: Option<LangStandardRequired<'t>>,
    pub lang_extensions: Option<LangExtensions<'t>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::common::source::SourceFromContent;
    use crate::doc::cmake_parse::tests::{quoted_token, quoted_tokens_vec, token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn try_compile() {
        let src = include_bytes!("../../../../../fixture/commands/project/try_compile");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::TryCompile(Box::new(TryCompile::SourcesAlt(
                    TryCompileProjectSourcesAlt {
                        compile_result_var: token(b"HAVE_PLWID"),
                        binary_dir: quoted_token(b"${CMAKE_BINARY_DIR}/temp"),
                        sources: SourceAlt::Source(quoted_token(
                            b"${CMAKE_SOURCE_DIR}/tests/test_plwid.c"
                        )),
                        cmake_flags: Some(quoted_tokens_vec([
                            b"-DINCLUDE_DIRECTORIES=${PLPLOT_INCLUDE_PATH}",
                            b"-DLINK_DIRECTORIES=${PLPLOT_LIB_PATH}"
                        ])),
                        compile_definitions: None,
                        link_options: None,
                        link_libraries: Some(tokens_vec([b"${PLPLOT_LIBRARY}"])),
                        output_variable: None,
                        copy_file: None,
                        lang_standard: None,
                        lang_standard_required: None,
                        lang_extensions: None,
                    }
                ))),
                Command::TryCompile(Box::new(TryCompile::Sources(TryCompileProjectSources {
                    compile_result_var: token(b"HAVE_PLWID"),
                    sources: vec![
                        Source::SourceFromContent(SourceFromContent {
                            name: token(b"aaa"),
                            content: token(b"bbb")
                        }),
                        Source::Sources(tokens_vec([b"ccc", b"ddd"]))
                    ],
                    log_description: Some(token(b"my log description")),
                    no_cache: true,
                    no_log: true,
                    cmake_flags: Some(quoted_tokens_vec([
                        b"-DINCLUDE_DIRECTORIES=${PLPLOT_INCLUDE_PATH}",
                        b"-DLINK_DIRECTORIES=${PLPLOT_LIB_PATH}"
                    ])),
                    compile_definitions: Some(tokens_vec([b"cd1", b"cd2"])),
                    link_options: Some(tokens_vec([b"opt1", b"opt2"])),
                    link_libraries: Some(tokens_vec([b"${PLPLOT_LIBRARY}", b"lib2"])),
                    output_variable: Some(token(b"VARIABLE1")),
                    copy_file: Some(CopyFile {
                        file_name: token(b"file1"),
                        copy_file_error: Some(token(b"fileError1"))
                    }),
                    lang_standard: None,
                    lang_standard_required: None,
                    lang_extensions: None,
                }))),
            ])
        )
    }
}
