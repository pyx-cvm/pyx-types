use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::common::Invalid;

/// A user identifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub struct Uid(u32);

impl Deref for Uid {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Uid> for u32 {
    fn from(uid: Uid) -> Self {
        uid.0
    }
}

impl TryFrom<u32> for Uid {
    type Error = Invalid<u32>;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 1000 {
            return Err(Invalid(value));
        }

        Ok(Self(value))
    }
}
