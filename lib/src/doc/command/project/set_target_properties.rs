use cmake_parser_derive::CMake;

use crate::{
    command::common::Property,
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Sets properties on targets.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/set_target_properties.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "targets")]
pub struct SetTargetProperties<'t> {
    #[cmake(rename = "")]
    pub targets: Vec<Token<'t>>,
    pub properties: Vec<Property<'t>>,
}

impl<'t> ToCommandScope for SetTargetProperties<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn set_target_properties() {
        let src = include_bytes!("../../../../../fixture/commands/project/set_target_properties");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::SetTargetProperties(Box::new(
                SetTargetProperties {
                    targets: tokens_vec([b"_python"]),
                    properties: vec![Property {
                        prop: token(b"OUTPUT_NAME"),
                        value: token(b"${PROJECT_NAME}"),
                    }]
                }
            )),])
        )
    }
}
