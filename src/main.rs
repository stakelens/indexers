mod db;
mod handlers;

use ghost_crab::prelude::*;
use handlers::coinbase;
use handlers::etherfi;
use handlers::lido;
use handlers::mantle;
use handlers::prices;
use handlers::renzo;
use handlers::rocketpool;
use handlers::stader;
use handlers::stakewise;
use handlers::swell;

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut indexer = ghost_crab::Indexer::new().unwrap();

    indexer
        .load_event_handler(stakewise::VaultsRegistry::new())
        .await
        .unwrap();

    indexer
        .load_event_handler(etherfi::EtherFiTVLUpdated::new())
        .await
        .unwrap();

    indexer
        .load_block_handler(rocketpool::RocketPoolBlockHandler::new())
        .await
        .unwrap();

    indexer
        .load_block_handler(swell::SwellBlockHandler::new())
        .await
        .unwrap();

    indexer
        .load_block_handler(lido::LidoBlockHandler::new())
        .await
        .unwrap();

    indexer
        .load_block_handler(renzo::RenzoBlockHandler::new())
        .await
        .unwrap();

    indexer
        .load_block_handler(mantle::MantleBlockHandler::new())
        .await
        .unwrap();

    indexer
        .load_block_handler(stader::StaderBlockHandler::new())
        .await
        .unwrap();

    indexer
        .load_block_handler(prices::ETHUSDCUniswapTWAP::new())
        .await
        .unwrap();

    indexer
        .load_block_handler(prices::RPLUSDCUniswapTWAP::new())
        .await
        .unwrap();

    indexer
        .load_block_handler(coinbase::CbETHBlockHandler::new())
        .await
        .unwrap();

    indexer
        .load_block_handler(coinbase::CbETHBaseBlockHandler::new())
        .await
        .unwrap();

    indexer.start().await.unwrap();
}
