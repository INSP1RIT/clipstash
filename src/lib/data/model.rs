use crate::data::DbId;
use crate::{ClipError,ShortCode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;
use crate::domain::clip::field::ClipId;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub clip_id: String,
    pub shortcode: String,
    pub content: String,
    pub title: Option<String>,
    pub posted: NaiveDateTime,
    pub expires: Option<NaiveDateTime>,
    pub password: Option<String>,
    pub hits: i64,
}


impl TryFrom<Clip> for crate::domain::clip::Clip {
    type Error = ClipError;

    fn try_from(value: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        Ok( Self {
             clip_id: ClipId::new(DbId::try_from(value.clip_id.as_str())?),
             shortcode: String,
             content: String,
             title: Option<String>,
             posted: NaiveDateTime,
             expires: Option<NaiveDateTime>,
             password: Option<String>,
             hits: i64,
        })
    }
}