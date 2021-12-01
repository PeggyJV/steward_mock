use serde::Serialize;

use crate::schema::types::{Mock, SchemaType};

#[derive(Debug, Serialize)]
pub struct UniswapV3Data {
    token_id: u128,
    tick_upper: i32,
    tick_lower: i32,
    weight: u32,
}

impl Mock for UniswapV3Data {
    fn generate_data() -> Vec<Self> {
        vec![UniswapV3Data {
            token_id: 1,
            tick_upper: 100,
            tick_lower: 0,
            weight: 42,
        }]
    }

    fn get_schema_type() -> SchemaType {
        SchemaType::UniswapV3
    }
}
