use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Unit {
    #[default]
    B,
    KB,
    KiB,
    MB,
    MiB,
    GB,
    GiB,
    TB,
    TiB,
    PB,
    PiB,
    EB,
    EiB,
}

impl Unit {
    pub(crate) const SORTED: &'static [Self] = &[
        Self::B,
        Self::KB,
        Self::KiB,
        Self::MB,
        Self::MiB,
        Self::GB,
        Self::GiB,
        Self::TB,
        Self::TiB,
        Self::PB,
        Self::PiB,
        Self::EB,
        Self::EiB,
    ];

    #[inline]
    pub fn scaled(self, count: u64) -> Result<super::Size, ParseIntError> {
        self.bytes()
            .checked_mul(count)
            .map(super::Size::from)
            .ok_or_else(|| u8::from_str_radix("12345", 10).unwrap_err())
    }

    #[inline]
    pub const fn bytes(self) -> u64 {
        match self {
            Unit::B => 1,
            Unit::KB => 1_000,
            Unit::KiB => 1 << 10,
            Unit::MB => 1_000_000,
            Unit::MiB => 1 << 20,
            Unit::GB => 1_000_000_000,
            Unit::GiB => 1 << 30,
            Unit::TB => 1_000_000_000_000,
            Unit::TiB => 1 << 40,
            Unit::PB => 1_000_000_000_000_000,
            Unit::PiB => 1 << 50,
            Unit::EB => 1_000_000_000_000_000_000,
            Unit::EiB => 1 << 60,
        }
    }
}

impl TryFrom<&str> for Unit {
    type Error = ParseIntError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for Unit {
    type Error = ParseIntError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<Unit> for String {
    #[inline]
    fn from(unit: Unit) -> Self {
        unit.to_string()
    }
}

impl AsRef<str> for Unit {
    #[inline]
    fn as_ref(&self) -> &str {
        match self {
            Unit::B => "",
            Unit::KB => "KB",
            Unit::KiB => "KiB",
            Unit::MB => "MB",
            Unit::MiB => "MiB",
            Unit::GB => "GB",
            Unit::GiB => "GiB",
            Unit::TB => "TB",
            Unit::TiB => "TiB",
            Unit::PB => "PB",
            Unit::PiB => "PiB",
            Unit::EB => "EB",
            Unit::EiB => "EiB",
        }
    }
}

impl Display for Unit {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for Unit {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "B" => Ok(Unit::B),
            "KB" => Ok(Unit::KB),
            "KiB" => Ok(Unit::KiB),
            "MB" => Ok(Unit::MB),
            "MiB" => Ok(Unit::MiB),
            "GB" => Ok(Unit::GB),
            "GiB" => Ok(Unit::GiB),
            "TB" => Ok(Unit::TB),
            "TiB" => Ok(Unit::TiB),
            "PB" => Ok(Unit::PB),
            "PiB" => Ok(Unit::PiB),
            "EB" => Ok(Unit::EB),
            "EiB" => Ok(Unit::EiB),
            _ => Err(u64::from_str("X").unwrap_err()),
        }
    }
}

#[cfg(test)]
#[rstest::rstest]
#[case("", Unit::B, "")]
#[case("B", Unit::B, "")]
#[case("KB", Unit::KB, "KB")]
#[case("KiB", Unit::KiB, "KiB")]
#[case("MB", Unit::MB, "MB")]
#[case("MiB", Unit::MiB, "MiB")]
#[case("GB", Unit::GB, "GB")]
#[case("GiB", Unit::GiB, "GiB")]
#[case("TB", Unit::TB, "TB")]
#[case("TiB", Unit::TiB, "TiB")]
#[case("PB", Unit::PB, "PB")]
#[case("PiB", Unit::PiB, "PiB")]
#[case("EB", Unit::EB, "EB")]
#[case("EiB", Unit::EiB, "EiB")]
fn codec(#[case] input: &str, #[case] unit: Unit, #[case] output: &str) {
    assert_eq!(unit, input.parse().unwrap());
    assert_eq!(output, unit.as_ref());
}
