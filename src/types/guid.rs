use bincode::{Decode, Encode};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Pod, Zeroable, Decode, Encode)]
pub struct Guid(pub [u8; 16]);

impl Guid {
    pub fn as_uuid(&self) -> Uuid {
        Uuid::from_bytes_le(self.0)
    }
}

impl From<Uuid> for Guid {
    fn from(uuid: Uuid) -> Self {
        Self(uuid.to_bytes_le())
    }
}

impl FromStr for Guid {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)?;
        Ok(uuid.into())
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_uuid())
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Guid(\"{}\")", self.as_uuid())
    }
}

impl Serialize for Guid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.as_uuid().to_string())
    }
}

impl<'de> Deserialize<'de> for Guid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let uuid = Uuid::parse_str(&s).map_err(serde::de::Error::custom)?;
        Ok(uuid.into())
    }
}
