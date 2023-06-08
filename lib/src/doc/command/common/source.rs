use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum Source<'t> {
    Sources(Vec<Token<'t>>),
    SourceFromContent(SourceFromContent<'t>),
    SourceFromVar(SourceFromVar<'t>),
    SourceFromFile(SourceFromFile<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct SourceFromContent<'t> {
    pub name: Token<'t>,
    pub content: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct SourceFromVar<'t> {
    pub name: Token<'t>,
    pub var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct SourceFromFile<'t> {
    pub name: Token<'t>,
    pub path: Token<'t>,
}
