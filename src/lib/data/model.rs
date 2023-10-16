use crate::data::DbId;
use crate::{ClipError, ShortCode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;
use crate::domain::clip::field::ClipId;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
}


impl TryFrom<Clip> for crate::domain::clip::Clip {
    type Error = ClipError;

    fn try_from(value: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        use std::str::FromStr;
        Ok(Self {
            clip_id: ClipId::new(DbId::try_from(value.clip_id.as_str())?),
            shortcode: ShortCode::from(value.shortcode),
            content: field::Content::new(value.content.as_str())?,
            title: field::Title::new(value.title),
            posted: field::Posted::new(Time::from_native_utc(value.posted)),
            expires: field::Expires::new(value.expires.map(Time::from_native_utc)),
            password: field::Password::new(value.password.unwrap_or_default())?,
            hits: field::Hits::new(u64::try_from(value.hits)?),
        })
    }
}