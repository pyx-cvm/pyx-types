mod uid;

pub use uid::Uid;

use std::{collections::BTreeMap, num::NonZeroU16};

use serde::{Deserialize, Serialize};

use super::common::{Id, ImageOr, Port};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct System {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ports: BTreeMap<NonZeroU16, Port>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub users: BTreeMap<Id, UserDef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserDef {
    pub uid: Uid,
    pub manifest: ImageOr<super::user::User>,
}
