use std::time::Duration;

mod rocketpool;

#[tokio::main]
async fn main() {
    rocketpool::MinipoolCreatedHandler::init();
    tokio::time::sleep(Duration::from_secs(1000)).await
}
