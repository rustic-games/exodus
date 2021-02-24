use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;
use std::ops::Deref;

// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MaxHeight {
    pub cm: NonZeroU16,
}

impl From<NonZeroU16> for MaxHeight {
    fn from(cm: NonZeroU16) -> Self {
        Self { cm }
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Name(pub String);

impl From<String> for Name {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
