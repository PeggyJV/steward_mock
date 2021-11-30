use serde::Serialize;

use super::types::Generate;

#[derive(Debug, Serialize)]
pub struct UniswapV3Data {
    token_id: u128,
    tick_upper: i32,
    tick_lower: i32,
    weight: u32,
}

impl Generate for UniswapV3Data {
    fn generate() -> Self {
        UniswapV3Data {
            token_id: 1,
            tick_upper: 100,
            tick_lower: 0,
            weight: 42,
        }
    }
}
