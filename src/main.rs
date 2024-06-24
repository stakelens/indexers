mod db;
mod handlers;

use ghost_crab::prelude::*;
use handlers::rocketpool;

#[tokio::main]
async fn main() {
    let mut indexer = ghost_crab::Indexer::new();

    indexer.load(rocketpool::MinipoolCreated::new()).await;

    indexer.start().await;
}
