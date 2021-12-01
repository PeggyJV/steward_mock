use axum::{extract, response};

use crate::schema::types::{Mock, RequestBody, ResponseBody};

pub async fn handle_get_data<T>(
    extract::Json(payload): extract::Json<RequestBody>,
) -> response::Json<ResponseBody<T>>
where
    T: Mock,
{
    response::Json(ResponseBody::<T> {
        cellar_id: payload.cellar_id,
        check_frequency: 1,
        created_timestamp: chrono::offset::Utc::now(),
        schema_type: T::get_schema_type(),
        data: T::generate_data(),
    })
}
