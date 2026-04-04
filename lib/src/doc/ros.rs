use std::borrow::Cow;

use crate::Token;

use super::RawCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RosCommand<'t> {
    AmentPackage,
    CatkinPackage,
    AmentTargetDependencies(AmentTargetDependencies<'t>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmentTargetDependencies<'t> {
    pub target: Token<'t>,
    pub dependencies: Vec<Token<'t>>,
}

impl<'t> RosCommand<'t> {
    pub fn from_raw(raw: &RawCommand<'t>) -> Option<Self> {
        match ascii_lowercase(raw.identifier.as_ref()).as_ref() {
            b"ament_package" => Some(Self::AmentPackage),
            b"catkin_package" => Some(Self::CatkinPackage),
            b"ament_target_dependencies" => {
                let (target, rest) = raw.tokens.split_first()?;
                let dependencies = rest
                    .iter()
                    .filter(|token| !is_ament_keyword(token.as_ref()))
                    .cloned()
                    .collect();
                Some(Self::AmentTargetDependencies(AmentTargetDependencies {
                    target: target.clone(),
                    dependencies,
                }))
            }
            _ => None,
        }
    }
}

fn is_ament_keyword(token: &[u8]) -> bool {
    matches!(token, b"SYSTEM" | b"PUBLIC" | b"INTERFACE")
}

fn ascii_lowercase(bytes: &[u8]) -> Cow<'_, [u8]> {
    if bytes.iter().any(u8::is_ascii_uppercase) {
        Cow::Owned(bytes.to_ascii_lowercase())
    } else {
        Cow::Borrowed(bytes)
    }
}
