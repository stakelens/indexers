mod db;
mod handlers;

use ghost_crab::prelude::*;
// use handlers::rocketpool;
use handlers::stakewise;

#[tokio::main]
async fn main() {
    let mut indexer = ghost_crab::Indexer::new();

    // indexer.load(rocketpool::MinipoolCreated::new()).await;
    // indexer.load(etherfi::EtherFiTVLUpdated::new()).await;
    indexer.load(stakewise::VaultsRegistry::new()).await;

    indexer.start().await;
}
