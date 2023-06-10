use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Perform the CTest Submit Step as a Dashboard Client.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_submit.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum CTestSubmit<'t> {
    #[cmake(rename = "CDASH_UPLOAD", transparent)]
    CDashUpload(CDashUpload<'t>),
    Dashboard(CTestSubmitDashboard<'t>),
}

impl<'t> ToCommandScope for CTestSubmit<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::CTest
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", allow_empty)]
pub struct CTestSubmitDashboard<'t> {
    pub parts: Option<Vec<Token<'t>>>,
    pub files: Option<Vec<Token<'t>>>,
    pub submit_url: Option<Token<'t>>,
    pub build_id: Option<Token<'t>>,
    pub httpheader: Option<Token<'t>>,
    pub retry_count: Option<Token<'t>>,
    pub retry_delay: Option<Token<'t>>,
    pub return_value: Option<Token<'t>>,
    pub capture_cmake_error: Option<Token<'t>>,
    pub quiet: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct CDashUpload<'t> {
    #[cmake(positional)]
    pub file: Token<'t>,
    pub cdash_upload_type: Option<Token<'t>>,
    pub submit_url: Option<Token<'t>>,
    pub build_id: Option<Token<'t>>,
    pub httpheader: Option<Token<'t>>,
    pub retry_count: Option<Token<'t>>,
    pub retry_delay: Option<Token<'t>>,
    pub return_value: Option<Token<'t>>,
    pub quiet: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ctest_submit() {
        let src = include_bytes!("../../../../../fixture/commands/ctest/ctest_submit");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CTestSubmit(Box::new(CTestSubmit::Dashboard(CTestSubmitDashboard {
                    parts: None,
                    files: None,
                    submit_url: None,
                    build_id: None,
                    httpheader: None,
                    retry_count: None,
                    retry_delay: None,
                    return_value: None,
                    capture_cmake_error: None,
                    quiet: false,
                }))),
                Command::CTestSubmit(Box::new(CTestSubmit::Dashboard(CTestSubmitDashboard {
                    parts: Some(tokens_vec([b"part1", b"part2", b"part3"])),
                    files: Some(tokens_vec([b"file1", b"file2"])),
                    submit_url: Some(token(b"submit_url1")),
                    build_id: Some(token(b"build_id1")),
                    httpheader: Some(token(b"httpheader1")),
                    retry_count: Some(token(b"retry_count1")),
                    retry_delay: Some(token(b"retry_delay1")),
                    return_value: Some(token(b"return_value1")),
                    capture_cmake_error: Some(token(b"capture_cmake_error1")),
                    quiet: true,
                }))),
                Command::CTestSubmit(Box::new(CTestSubmit::CDashUpload(CDashUpload {
                    file: token(b"file1"),
                    cdash_upload_type: Some(token(b"cdash_upload_type1")),
                    submit_url: Some(token(b"submit_url1")),
                    build_id: Some(token(b"build_id1")),
                    httpheader: Some(token(b"httpheader1")),
                    retry_count: Some(token(b"retry_count1")),
                    retry_delay: Some(token(b"retry_delay1")),
                    return_value: Some(token(b"return_value1")),
                    quiet: true,
                }))),
            ])
        )
    }
}
