use crate::{command::CommandParseError, Token};

/// Adds options to the compilation of source files.
///
/// Reference: https://cmake.org/cmake/help/v3.26/command/add_compile_options.html
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddCompileOptions<'t> {
    pub compile_options: Vec<Token<'t>>,
}

impl<'t> TryFrom<Vec<Token<'t>>> for AddCompileOptions<'t> {
    type Error = CommandParseError;

    fn try_from(compile_options: Vec<Token<'t>>) -> Result<Self, Self::Error> {
        Ok(Self { compile_options })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn add_compile_options() {
        let src = include_bytes!("../../../../fixture/commands/add_compile_options");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands().unwrap(),
            &[Command::AddCompileOptions(AddCompileOptions {
                compile_options: vec![
                    Token::text_node(&b"-foo"[..]),
                    Token::text_node(&b"-bar"[..])
                ]
            })]
        )
    }
}
