use std::borrow::Borrow;
use std::ops::Deref;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::Invalid;

/// a domain identifier
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Id(String);

impl Deref for Id {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<String> for Id {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Id {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Id {
    type Error = Invalid<String>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // username: 32, dns: 63, etc.; be conservative
        match value.len() {
            1..=32 => (),
            _ => return Err(Invalid(value)),
        }

        for c in value.chars() {
            match c {
                'a'..='z' | '0'..='9' | '-' => (),
                _ => return Err(Invalid(value)),
            }
        }

        if !value.starts_with(|c: char| c.is_ascii_alphabetic()) {
            return Err(Invalid(value));
        }

        if !value.ends_with(|c: char| c.is_ascii_alphanumeric()) {
            return Err(Invalid(value));
        }

        Ok(Self(value))
    }
}

impl FromStr for Id {
    type Err = Invalid<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.to_string().try_into()
    }
}

impl From<Id> for String {
    fn from(value: Id) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abc123", true)]
    #[case("a-b-c", true)]
    #[case("abc", true)]
    #[case("a1", true)]
    #[case("", false)]
    #[case("1abc", false)]
    #[case("abc-", false)]
    #[case("ABC", false)]
    #[case("a_b", false)]
    #[case(&"a".repeat(33), false)]
    fn parsing(#[case] input: &str, #[case] should_succeed: bool) {
        let result = Id::try_from(input.to_string());
        assert_eq!(result.is_ok(), should_succeed);
    }

    #[rstest]
    #[case("test-id")]
    #[case("abc123")]
    fn serde(#[case] id_str: &str) {
        let id = Id::try_from(id_str.to_string()).unwrap();
        let serialized = serde_yml::to_string(&id).unwrap();
        assert_eq!(serialized.trim(), id_str);

        let deserialized: Id = serde_yml::from_str(&serialized).unwrap();
        assert_eq!(id, deserialized);
    }
}
