pub mod system;
pub mod user;

use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "scope", rename_all = "kebab-case")]
#[allow(variant_size_differences)]
pub enum Scope {
    System(system::System),
    User(user::User),
}
