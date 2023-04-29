pub trait ToCommandScope {
    fn to_command_scope(&self) -> CommandScope;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CommandScope {
    Scripting,
    Project,
    CTest,
    Deprecated,
}
