use axum::{extract, response};
use serde::Serialize;

use crate::schema::types::{Mock, RequestBody, ResponseBody};

pub async fn handle_get_data<T>(
    extract::Json(_payload): extract::Json<RequestBody>,
) -> response::Json<ResponseBody<T>>
where
    T: Mock + Serialize,
{
    response::Json(ResponseBody::<T> {
        cellar_id: "whatever".to_string(),
        check_frequency: 1,
        created_timestamp: chrono::offset::Utc::now(),
        schema_type: T::get_schema_type(),
        data: T::generate_data(),
    })
}
