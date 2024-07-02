mod db;
mod handlers;

use ghost_crab::prelude::*;
use handlers::etherfi;
use handlers::lido;
use handlers::mantle;
use handlers::renzo;
use handlers::rocketpool;
use handlers::stakewise;
use handlers::swell;

#[tokio::main]
async fn main() {
    let mut indexer = ghost_crab::Indexer::new();

    indexer
        .load_event_handler(stakewise::VaultsRegistry::new())
        .await;
    indexer
        .load_event_handler(etherfi::EtherFiTVLUpdated::new())
        .await;
    indexer
        .load_block_handler(rocketpool::RocketPoolBlockHandler::new())
        .await;
    indexer
        .load_block_handler(swell::SwellBlockHandler::new())
        .await;
    indexer
        .load_block_handler(lido::LidoBlockHandler::new())
        .await;
    indexer
        .load_block_handler(renzo::RenzoBlockHandler::new())
        .await;
    indexer
        .load_block_handler(mantle::MantleBlockHandler::new())
        .await;

    indexer.start().await;
}
