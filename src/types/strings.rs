use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StringU16(pub Vec<u16>);  // Length-prefixed

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StringU16C(pub Vec<u16>); // Null-terminated

macro_rules! impl_u16_string_basics {
    ($type:ident) => {
        impl $type {
            pub fn as_string(&self) -> String {
                String::from_utf16_lossy(&self.0).trim_end_matches('\0').to_string()
            }
        }

        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_string())
            }
        }

        impl fmt::Debug for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}(\"{}\")", stringify!($type), self.as_string())
            }
        }

        impl Serialize for $type {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.as_string())
            }
        }

        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let s = String::deserialize(deserializer)?;
                let mut data: Vec<u16> = s.encode_utf16().collect();

                if stringify!($type) == "StringU16C" && data.last() != Some(&0) {
                    data.push(0);
                }

                Ok(Self(data))
            }
        }
    };
}

impl_u16_string_basics!(StringU16);
impl_u16_string_basics!(StringU16C);
