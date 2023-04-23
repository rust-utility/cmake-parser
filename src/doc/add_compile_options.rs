use crate::{parser::CommandInvocation, TextNode};

///
/// Reference: https://cmake.org/cmake/help/v3.0/command/add_compile_options.html
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddCompileOptions<TN> {
    pub compile_options: Vec<TN>,
}

impl<'tn, TN: TextNode<'tn>> From<&'tn CommandInvocation<'tn>> for AddCompileOptions<TN> {
    fn from(value: &'tn CommandInvocation<'tn>) -> Self {
        let compile_options = value.to_text_nodes();
        Self { compile_options }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn add_compile_options() {
        let src = include_bytes!("../../fixture/commands/add_compile_options");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Utf8Doc::from(&cmakelists);
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
