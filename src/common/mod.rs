pub mod port;

mod id;
mod size;
mod unit;

pub use id::Id;
pub use size::Size;
pub use unit::Unit;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageOr<T> {
    Image(oci_imgref::image::Image),
    Other(T),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Invalid<T>(pub T);

impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Invalid<T> {}

impl<T: std::fmt::Display> std::fmt::Display for Invalid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid: {}", self.0)
    }
}
