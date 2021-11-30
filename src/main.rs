use steward::server::server;

#[tokio::main]
async fn main() {
    server::serve().await;
}
