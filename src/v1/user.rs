use std::{collections::BTreeMap, num::NonZeroU16};

use oci_imgref::image::Image;
use serde::{Deserialize, Serialize};

use super::common::{Id, ImageOr, Port};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct User {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ports: BTreeMap<NonZeroU16, Port>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub pods: BTreeMap<Id, Pod>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Pod {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ports: BTreeMap<NonZeroU16, Port>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub containers: BTreeMap<Id, ImageOr<Container>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Container {
    pub image: Image,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    macro_rules! map {
        ($($k:expr => $v:expr),* $(,)?) => {{
            let mut map = BTreeMap::new();
            $(let _ = map.insert($k, $v);)*
            map
        }};
    }

    #[test]
    fn serde() {
        let rust = User {
            ports: map!(NonZeroU16::new(443).unwrap() => Port {
                name: "web".parse().unwrap(),
                port: None,
                prot: None
            }),

            pods: map!(
                Id::from_str("web").unwrap() => Pod {
                    ports: map!(NonZeroU16::new(443).unwrap() => Port {
                        name: "nginx".parse().unwrap(),
                        port: None,
                        prot: None
                    }),

                    containers: map!(
                        "nginx".parse().unwrap() => ImageOr::Image(Image::from_str("nginx:1.25-alpine").unwrap()),
                        "postgres".parse().unwrap() => ImageOr::Other(Container {
                            image: Image::from_str("postgres:16-alpine").unwrap(),
                        }),
                    ),
                }
            ),
        };

        let yaml = r#"
ports:
  443: web
pods:
  web:
    ports:
      443: nginx
    containers:
      nginx: nginx:1.25-alpine
      postgres:
        image: postgres:16-alpine
"#;
        assert_eq!(rust, serde_yml::from_str(yaml).unwrap());
        assert_eq!(yaml.trim(), serde_yml::to_string(&rust).unwrap().trim());
    }
}
