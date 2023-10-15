use crate::domain::clip::ClipError;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use derive_more::{Constructor};
use crate::domain::time::Time;


#[derive(Clone, Constructor, Debug, Serialize, Deserialize)]
pub struct Posted(Time);


impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    }
}