
use axum::{routing::get, Router};

use crate::schema::uniswap_v3::UniswapV3Data;

use super::handlers::handle_get_data;

pub async fn serve() {
    let router = Router::new().route("/uniswap-v3", get(|| async { handle_get_data::<UniswapV3Data>().await }));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
