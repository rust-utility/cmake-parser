use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Perform the CTest MemCheck Step as a Dashboard Client.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_memcheck.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", allow_empty)]
pub struct CTestMemCheck<'t> {
    #[cmake(rename = "BUILD")]
    pub build_dir: Option<Token<'t>>,
    pub append: bool,
    pub start: Option<Token<'t>>,
    pub end: Option<Token<'t>>,
    pub stride: Option<Token<'t>>,
    pub exclude: Option<Token<'t>>,
    pub include: Option<Token<'t>>,
    pub exclude_label: Option<Token<'t>>,
    pub include_label: Option<Token<'t>>,
    pub exclude_fixture: Option<Token<'t>>,
    pub exclude_fixture_setup: Option<Token<'t>>,
    pub exclude_fixture_cleanup: Option<Token<'t>>,
    pub parallel_level: Option<Token<'t>>,
    pub resource_spec_file: Option<Token<'t>>,
    pub test_load: Option<Token<'t>>,
    pub schedule_random: Option<ScheduleRandom>,
    pub stop_on_failure: bool,
    pub stop_time: Option<Token<'t>>,
    pub return_value: Option<Token<'t>>,
    pub capture_cmake_error: Option<Token<'t>>,
    pub repeat: Option<Token<'t>>,
    pub output_junit: Option<Token<'t>>,
    pub defect_count: Option<Token<'t>>,
    pub quiet: bool,
}

impl<'t> ToCommandScope for CTestMemCheck<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::CTest
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum ScheduleRandom {
    On,
    Off,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ctest_memcheck() {
        let src = include_bytes!("../../../../../fixture/commands/ctest/ctest_memcheck");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CTestMemCheck(Box::new(CTestMemCheck {
                    build_dir: None,
                    append: false,
                    start: None,
                    end: None,
                    stride: None,
                    exclude: None,
                    include: None,
                    exclude_label: None,
                    include_label: None,
                    exclude_fixture: None,
                    exclude_fixture_setup: None,
                    exclude_fixture_cleanup: None,
                    parallel_level: None,
                    resource_spec_file: None,
                    test_load: None,
                    schedule_random: None,
                    stop_on_failure: false,
                    stop_time: None,
                    return_value: None,
                    capture_cmake_error: None,
                    repeat: None,
                    output_junit: None,
                    defect_count: None,
                    quiet: false,
                })),
                Command::CTestMemCheck(Box::new(CTestMemCheck {
                    build_dir: Some(token(b"build1")),
                    append: true,
                    start: Some(token(b"start1")),
                    end: Some(token(b"end1")),
                    stride: Some(token(b"stride1")),
                    exclude: Some(token(b"exclude1")),
                    include: Some(token(b"include1")),
                    exclude_label: Some(token(b"exclude_label1")),
                    include_label: Some(token(b"include_label1")),
                    exclude_fixture: Some(token(b"exclude_fixture1")),
                    exclude_fixture_setup: Some(token(b"exclude_fixture_setup1")),
                    exclude_fixture_cleanup: Some(token(b"exclude_fixture_cleanup1")),
                    parallel_level: Some(token(b"parallel_level1")),
                    resource_spec_file: Some(token(b"resource_spec_file1")),
                    test_load: Some(token(b"test_load1")),
                    schedule_random: Some(ScheduleRandom::Off),
                    stop_on_failure: true,
                    stop_time: Some(token(b"stop_time1")),
                    return_value: Some(token(b"return_value1")),
                    capture_cmake_error: Some(token(b"capture_cmake_error1")),
                    repeat: Some(token(b"repeat1")),
                    output_junit: Some(token(b"output_junit1")),
                    defect_count: Some(token(b"defect_count1")),
                    quiet: true,
                })),
            ])
        )
    }
}
