use chrono::{DateTime, Utc};
use serde::{self, Serializer};

// Code taken from serializer example for serizlizing a timestamp.
// This is used by serde when a chrono::DateTime<Utc> is marked with
// the serde directive.
const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(TIMESTAMP_FORMAT));
    serializer.serialize_str(&s)
}
