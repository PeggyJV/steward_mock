use crate::schema::timestamp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum SchemaType {
    UniswapV3,
}

pub trait Mock {
    fn generate_data() -> Vec<Self>
    where
        Self: Sized;

    fn get_schema_type() -> SchemaType;
}

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    pub cellar_id: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseBody<T> {
    pub cellar_id: String,
    pub check_frequency: u64,
    // see timestamp.rs
    #[serde(with = "timestamp")]
    pub created_timestamp: DateTime<Utc>,
    pub schema_type: SchemaType,
    pub data: Vec<T>,
}
