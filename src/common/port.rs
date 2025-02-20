use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};

use super::Id;

pub type Port = NonZeroU16;

#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    #[default]
    Tcp,
    Udp,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(from = "Parser", into = "Parser")]
pub struct Target {
    pub name: Id,
    pub port: Option<NonZeroU16>,
    pub prot: Option<Protocol>,
}

impl From<Parser> for Target {
    #[inline]
    fn from(parser: Parser) -> Self {
        let (name, port, prot) = match parser {
            Parser::TupleNamePortProt(name, port, prot) => (name, Some(port), Some(prot)),
            Parser::TupleNameProt(name, prot) => (name, None, Some(prot)),
            Parser::TupleNamePort(name, port) => (name, Some(port), None),
            Parser::TupleName((name,)) => (name, None, None),
            Parser::Name(name) => (name, None, None),
        };

        Self { name, port, prot }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
enum Parser {
    TupleNamePortProt(Id, NonZeroU16, Protocol),
    TupleNameProt(Id, Protocol),
    TupleNamePort(Id, NonZeroU16),
    TupleName((Id,)),
    Name(Id),
}

impl From<Target> for Parser {
    #[inline]
    fn from(port: Target) -> Self {
        match (port.name, port.port, port.prot) {
            (name, Some(port), Some(prot)) => Self::TupleNamePortProt(name, port, prot),
            (name, None, Some(prot)) => Self::TupleNameProt(name, prot),
            (name, Some(port), None) => Self::TupleNamePort(name, port),
            (name, None, None) => Self::Name(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("http", 0, None, "http", "http\n")]
    #[case("http", 80, None, "[http, 80]", "- http\n- 80\n")]
    #[case("http", 0, Some(Protocol::Tcp), "[http, tcp]", "- http\n- tcp\n")]
    #[case("http", 0, Some(Protocol::Udp), "[http, udp]", "- http\n- udp\n")]
    #[case(
        "http",
        80,
        Some(Protocol::Tcp),
        "[http, 80, tcp]",
        "- http\n- 80\n- tcp\n"
    )]
    #[case(
        "http",
        443,
        Some(Protocol::Udp),
        "[http, 443, udp]",
        "- http\n- 443\n- udp\n"
    )]
    fn serde(
        #[case] name: &str,
        #[case] port: u16,
        #[case] prot: Option<Protocol>,
        #[case] src: &str,
        #[case] dst: &str,
    ) {
        let port = Target {
            name: Id::try_from(name.to_string()).unwrap(),
            port: NonZeroU16::new(port),
            prot,
        };

        assert_eq!(port, serde_yml::from_str(&src).unwrap());
        assert_eq!(dst, serde_yml::to_string(&port).unwrap());
    }
}
