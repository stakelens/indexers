mod rocketpool;

#[tokio::main]
async fn main() {
    let mut indexer = ghost_crab::Indexer::new();
    indexer.load(rocketpool::MinipoolCreatedHandler::new());
    indexer.start().await;
}
