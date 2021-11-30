use crate::schema::timestamp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::uniswap_v3::UniswapV3Data;

#[derive(Debug, Deserialize, Serialize)]
pub enum SchemaType {
    UniswapV3,
}

pub trait Generate {
    fn generate() -> Self;
}

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    pub cellar_id: String,
    pub schema_type: SchemaType,
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

impl Into<ResponseBody<UniswapV3Data>> for UniswapV3Data {
    fn into(self) -> ResponseBody<UniswapV3Data> {
        ResponseBody::<UniswapV3Data> {
            cellar_id: "1:0x00000000000000000000000000000000".to_string(),
            check_frequency: 3600,
            created_timestamp: chrono::offset::Utc::now(),
            schema_type: SchemaType::UniswapV3,
            data: vec!(UniswapV3Data::generate()),
        }
    }
}
