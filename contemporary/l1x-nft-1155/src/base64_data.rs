use serde::{de::Error, Deserialize, Deserializer};
use std::str;

#[derive(Debug)]
pub struct Base64Data(Vec<u8>);

impl<'de> Deserialize<'de> for Base64Data {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis;
        impl serde::de::Visitor<'_> for Vis {
            type Value = Base64Data;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a base64 string")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                base64::decode(v).map(Base64Data).map_err(Error::custom)
            }
        }
        deserializer.deserialize_str(Vis)
    }
}
