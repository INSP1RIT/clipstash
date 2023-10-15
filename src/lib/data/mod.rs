use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
use std::str::FromStr;

#[derive(From, Clone, Debug, Display, Serialize, Deserialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(s)?))
    }
}