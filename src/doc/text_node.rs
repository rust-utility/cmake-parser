use std::borrow::Cow;
use std::fmt::{self, Display};

pub trait TextNode<'tn>: Display {
    fn text_node<T>(bytes: T) -> Self
    where
        T: Into<Cow<'tn, [u8]>>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Utf8TextNode<'a> {
    bytes: Cow<'a, [u8]>,
}

impl<'a> TextNode<'a> for Utf8TextNode<'a> {
    fn text_node<T>(bytes: T) -> Self
    where
        T: Into<Cow<'a, [u8]>>,
    {
        Utf8TextNode {
            bytes: bytes.into(),
        }
    }
}

impl<'a> Display for Utf8TextNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes))
    }
}
