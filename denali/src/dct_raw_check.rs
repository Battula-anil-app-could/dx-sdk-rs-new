use super::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};
use std::{collections::BTreeMap, fmt};

pub enum CheckDctRaw {
    Unspecified,
    Star,
    Equal(BTreeMap<String, CheckBytesValueRaw>),
}

impl CheckDctRaw {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckDctRaw::Star)
    }

    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckDctRaw::Unspecified)
    }
}

impl Default for CheckDctRaw {
    fn default() -> Self {
        CheckDctRaw::Unspecified
    }
}

impl Serialize for CheckDctRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckDctRaw::Unspecified => {
                // empty map, just in case
                // won't get serialized anyway
                let map = serializer.serialize_map(Some(0))?;
                map.end()
            },
            CheckDctRaw::Star => serializer.serialize_str("*"),
            CheckDctRaw::Equal(m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            },
        }
    }
}

struct CheckDctRawVisitor;

impl<'de> Visitor<'de> for CheckDctRawVisitor {
    type Value = CheckDctRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of dct check")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckDctRaw::Star)
        } else {
            Err(de::Error::custom("only '*' allowed as dct string value"))
        }
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = BTreeMap::<String, CheckBytesValueRaw>::new();

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(CheckDctRaw::Equal(map))
    }
}

impl<'de> Deserialize<'de> for CheckDctRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckDctRawVisitor)
    }
}
