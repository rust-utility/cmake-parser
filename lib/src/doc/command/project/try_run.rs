use cmake_parser_derive::CMake;

use crate::{
    command::common::{
        CopyFile, LangExtensions, LangStandard, LangStandardRequired, Source, SourceAlt,
    },
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Try compiling and then running some code.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/try_run.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum TryRun<'t> {
    Regular(TryRunRegular<'t>),
    Alt(TryRunAlt<'t>),
}

impl<'t> ToCommandScope for TryRun<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "sources")]
pub struct TryRunRegular<'t> {
    #[cmake(positional)]
    pub run_result_var: Token<'t>,
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
    pub compile_output_variable: Option<Token<'t>>,
    pub copy_file: Option<CopyFile<'t>>,
    pub lang_standard: Option<LangStandard<'t>>,
    pub lang_standard_required: Option<LangStandardRequired<'t>>,
    pub lang_extensions: Option<LangExtensions<'t>>,

    pub run_output_variable: Option<Token<'t>>,
    pub run_output_stdout_variable: Option<Token<'t>>,
    pub run_output_stderr_variable: Option<Token<'t>>,
    pub working_directory: Option<Token<'t>>,
    pub args: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "sources")]
pub struct TryRunAlt<'t> {
    #[cmake(positional)]
    pub run_result_var: Token<'t>,
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
    pub compile_output_variable: Option<Token<'t>>,
    pub copy_file: Option<CopyFile<'t>>,
    pub lang_standard: Option<LangStandard<'t>>,
    pub lang_standard_required: Option<LangStandardRequired<'t>>,
    pub lang_extensions: Option<LangExtensions<'t>>,

    pub run_output_variable: Option<Token<'t>>,
    pub output_variable: Option<Token<'t>>,
    pub working_directory: Option<Token<'t>>,
    pub args: Option<Vec<Token<'t>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn try_run() {
        let src = include_bytes!("../../../../../fixture/commands/project/try_run");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::TryRun(Box::new(TryRun::Alt(TryRunAlt {
                run_result_var: token(b"test_run_result"),
                compile_result_var: token(b"test_compile_result"),
                binary_dir: token(b"${CMAKE_CURRENT_BINARY_DIR}/"),
                sources: SourceAlt::Source(token(b"${CMAKE_CURRENT_SOURCE_DIR}/test.cpp")),
                cmake_flags: None,
                compile_definitions: None,
                link_options: None,
                link_libraries: None,
                compile_output_variable: Some(token(b"test_compile_output")),
                copy_file: None,
                lang_standard: None,
                lang_standard_required: None,
                lang_extensions: None,
                run_output_variable: Some(token(b"test_run_output")),
                output_variable: None,
                working_directory: None,
                args: None,
            }))),])
        )
    }
}
