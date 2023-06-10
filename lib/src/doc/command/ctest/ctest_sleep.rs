use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Sleeps for some amount of time
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_sleep.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum CTestSleep<'t> {
    Time(CTestSleepTime<'t>),
    Seconds(CTestSleepSeconds<'t>),
}

impl<'t> ToCommandScope for CTestSleep<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::CTest
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CTestSleepTime<'t> {
    pub time1: Token<'t>,
    pub duration: Token<'t>,
    pub time2: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CTestSleepSeconds<'t> {
    pub seconds: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ctest_sleep() {
        let src = include_bytes!("../../../../../fixture/commands/ctest/ctest_sleep");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CTestSleep(Box::new(CTestSleep::Seconds(CTestSleepSeconds {
                    seconds: token(b"100"),
                }))),
                Command::CTestSleep(Box::new(CTestSleep::Time(CTestSleepTime {
                    time1: token(b"100"),
                    duration: token(b"200"),
                    time2: token(b"300"),
                }))),
            ])
        )
    }
}
