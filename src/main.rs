mod db;
mod handlers;

use ghost_crab::prelude::*;
use handlers::stakewise;

#[tokio::main]
async fn main() {
    let mut indexer = ghost_crab::Indexer::new();

    indexer.load(stakewise::VaultsRegistry::new()).await;

    indexer.start().await;
}
