mod uid;

pub use uid::Uid;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::common::port::{Port, Target};
use crate::common::{Id, ImageOr};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct System {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ports: BTreeMap<Port, Target>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub users: BTreeMap<Id, UserDef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserDef {
    pub uid: Uid,
    pub manifest: ImageOr<super::user::User>,
}
