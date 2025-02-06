use std::hash::Hasher;
use std::str::FromStr;
use std::{hash::Hash, num::ParseIntError};

use serde::{Deserialize, Serialize};

use super::Unit;

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Size(u64, Unit);

impl TryFrom<String> for Size {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<Size> for String {
    fn from(value: Size) -> Self {
        value.to_string()
    }
}

impl From<Size> for u64 {
    fn from(size: Size) -> Self {
        size.0 * size.1.bytes()
    }
}

impl From<u64> for Size {
    fn from(bytes: u64) -> Self {
        for unit in Unit::SORTED.iter().rev() {
            if bytes % unit.bytes() == 0 {
                return Size(bytes / unit.bytes(), *unit);
            }
        }

        unreachable!()
    }
}

impl Eq for Size {}
impl PartialEq<Size> for Size {
    #[inline]
    fn eq(&self, other: &Size) -> bool {
        u64::from(*self) == u64::from(*other)
    }
}

impl PartialEq<Size> for u64 {
    #[inline]
    fn eq(&self, other: &Size) -> bool {
        *self == u64::from(*other)
    }
}

impl PartialEq<u64> for Size {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        u64::from(*self) == *other
    }
}

impl PartialOrd<Size> for Size {
    #[inline]
    fn partial_cmp(&self, other: &Size) -> Option<std::cmp::Ordering> {
        u64::from(*self).partial_cmp(&u64::from(*other))
    }
}

impl PartialOrd<Size> for u64 {
    #[inline]
    fn partial_cmp(&self, other: &Size) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&u64::from(*other))
    }
}

impl PartialOrd<u64> for Size {
    #[inline]
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        u64::from(*self).partial_cmp(other)
    }
}

impl Ord for Size {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        u64::from(*self).cmp(&u64::from(*other))
    }
}

impl Hash for Size {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        u64::from(*self).hash(state);
    }
}

impl std::fmt::Display for Size {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl FromStr for Size {
    type Err = ParseIntError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.chars().position(|c| !c.is_digit(10)).unwrap_or(s.len());
        let unit: Unit = s[i..].trim().parse()?;
        unit.scaled(s[..i].parse()?)
    }
}

#[cfg(test)]
#[rstest::rstest]
#[case("7", Size(7, Unit::B), "7")]
#[case("7B", Size(7, Unit::B), "7")]
#[case("7KB", Size(7, Unit::KB), "7KB")]
#[case("7KiB", Size(7, Unit::KiB), "7KiB")]
#[case("7MB", Size(7, Unit::MB), "7MB")]
#[case("7MiB", Size(7, Unit::MiB), "7MiB")]
#[case("7GB", Size(7, Unit::GB), "7GB")]
#[case("7GiB", Size(7, Unit::GiB), "7GiB")]
#[case("7TB", Size(7, Unit::TB), "7TB")]
#[case("7TiB", Size(7, Unit::TiB), "7TiB")]
#[case("7PB", Size(7, Unit::PB), "7PB")]
#[case("7PiB", Size(7, Unit::PiB), "7PiB")]
#[case("7EB", Size(7, Unit::EB), "7EB")]
#[case("7EiB", Size(7, Unit::EiB), "7EiB")]
#[case("7 B", Size(7, Unit::B), "7")]
#[case("7 MiB", Size(7, Unit::MiB), "7MiB")]
fn codec(#[case] input: &str, #[case] size: Size, #[case] output: &str) {
    assert_eq!(size, Size::from_str(input).unwrap());
    assert_eq!(output, size.to_string());
}
