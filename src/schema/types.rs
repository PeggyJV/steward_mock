use crate::schema::timestamp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum SchemaType {
    UniswapV3(),
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub cellar_id: String,
    pub schema_type: SchemaType,
}

#[derive(Debug, Serialize)]
pub struct Response<T> {
    pub cellar_id: String,
    pub check_frequency: u64,
    // see timestamp.rs
    #[serde(with = "timestamp")]
    pub created_timestamp: DateTime<Utc>,
    pub schema_type: SchemaType,
    pub data: Vec<T>,
}

// {
//     "cellar_id": ...,
//     "data": [{}],
//     "created_timestamp": "...",
//     "check_frequency": 3600,
//     "schema_type": "UNIv3",
// }
