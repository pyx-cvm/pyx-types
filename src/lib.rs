#![forbid(unsafe_code, clippy::expect_used, clippy::panic)]
#![deny(
    clippy::all,
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    missing_copy_implementations,
    missing_debug_implementations,
    noop_method_call,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    single_use_lifetimes,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_code,
    unreachable_patterns,
    unstable_features,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

pub mod v1;

use serde::{Deserialize, Serialize};

pub type Profile = v1::common::ImageOr<Manifest>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "version", rename_all = "kebab-case")]
pub enum Manifest {
    V1(v1::Scope),
}
