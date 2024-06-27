mod db;
mod handlers;

use ghost_crab::prelude::*;
use handlers::stakewise;
use handlers::rocketpool;
use handlers::etherfi;

#[tokio::main]
async fn main() {
    let mut indexer = ghost_crab::Indexer::new();

    indexer.load_event_handler(stakewise::VaultsRegistry::new()).await;
    indexer.load_event_handler(etherfi::EtherFiTVLUpdated::new()).await;
    indexer.load_block_handler(rocketpool::RocketPoolBlockHandler::new()).await;

    indexer.start().await;
}
