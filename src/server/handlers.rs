use axum::Json;
use serde::Serialize;

use crate::schema::types::{ResponseBody, Generate};

pub async fn handle_get_data<T>() -> Json<ResponseBody<T>>
where
    T: Generate + Into<ResponseBody<T>> + Serialize
{
    let response: ResponseBody<T> = T::generate().into();

    Json(response)
}
