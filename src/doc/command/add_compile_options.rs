use crate::{command::CommandParseError, TextNode};

/// Adds options to the compilation of source files.
///
/// Reference: https://cmake.org/cmake/help/v3.26/command/add_compile_options.html
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddCompileOptions<TN> {
    pub compile_options: Vec<TN>,
}

impl<'tn, TN: TextNode<'tn>> TryFrom<Vec<TN>> for AddCompileOptions<TN> {
    type Error = CommandParseError;

    fn try_from(compile_options: Vec<TN>) -> Result<Self, Self::Error> {
        Ok(Self { compile_options })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn add_compile_options() {
        let src = include_bytes!("../../../fixture/commands/add_compile_options");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Utf8Doc::try_from(&cmakelists).expect("valid cmake document");
        assert_eq!(
            doc.commands(),
            &[Command::AddCompileOptions(AddCompileOptions {
                compile_options: vec![
                    Utf8TextNode::text_node(&b"-foo"[..]),
                    Utf8TextNode::text_node(&b"-bar"[..])
                ]
            })]
        )
    }
}
