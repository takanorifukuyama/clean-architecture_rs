use std::str::FromStr;

use anyhow::Result;
use derive_more::Display;
use serde::{de, Deserialize, Serialize, Serializer};
use ulid::Ulid;

use crate::util::myerror::MyError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Display)]
pub struct UserId(Ulid);

impl UserId {
    pub fn new(s: &str) -> Result<Self> {
        Ok(UserId(
            Ulid::from_string(s).map_err(|_| MyError::type_error("The is is wrong"))?,
        ))
    }
}

impl Default for UserId {
    fn default() -> Self {
        UserId(Ulid::new())
    }
}

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl<'de> de::Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(&s).map_err(de::Error::custom)
    }
}

impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
pub struct MailAddress(String);

impl FromStr for MailAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(MailAddress(s.to_string()))
    }
}
