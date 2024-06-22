use std::time::Duration;
mod handlers;

#[tokio::main]
async fn main() {
    handlers::rocketpool::MinipoolCreated::init();
    tokio::time::sleep(Duration::from_secs(1000)).await
}
