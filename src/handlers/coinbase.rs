use alloy::{eips::BlockId, sol};
use ghost_crab::prelude::*;
use log::{error, info};

use crate::db;

sol!(
    #[sol(rpc)]
    UpgradeableOptimismMintableERC20,
    "abis/coinbase/UpgradeableOptimismMintableERC20.json"
);

const COINBASE_ETH: Address = address!("Be9895146f7AF43049ca1c1AE358B0541Ea49704");

#[block_handler(CbETH)]
async fn CbETHBlockHandler(ctx: BlockContext) {
    let coinbase_eth_contract = UpgradeableOptimismMintableERC20::new(COINBASE_ETH, &ctx.provider);

    let pooled_eth = coinbase_eth_contract
        .totalSupply()
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    let db = db::get().await;

    let block_number = ctx.block_number as i64;
    let eth_supply = pooled_eth._0.to_string();

    let block = ctx.block(false).await.unwrap().unwrap();
    let block_timestamp = block.header.timestamp as i64;

    let result = sqlx::query!(
        r#"INSERT INTO "CbETH" (block_number, block_timestamp, eth)
           VALUES ($1, $2, $3)
           ON CONFLICT (block_number) DO NOTHING"#,
        block_number,
        block_timestamp,
        eth_supply,
    )
    .execute(db)
    .await;

    match result {
        Ok(_) => {
            info!("Successfully indexed Lido data for block {}", block_number);
        }
        Err(e) => {
            error!(
                "Error indexing Lido data for block {}: {:?}",
                block_number, e
            );
        }
    }
}

const COINBASE_BASE: Address = address!("2Ae3F1Ec7F1F5012CFEab0185bfc7aa3cf0DEc22");

#[block_handler(CbETHBase)]
async fn CbETHBaseBlockHandler(ctx: BlockContext) {
    let coinbase_eth_contract = UpgradeableOptimismMintableERC20::new(COINBASE_BASE, &ctx.provider);

    let pooled_eth = coinbase_eth_contract
        .totalSupply()
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    let db = db::get().await;

    let block_number = ctx.block_number as i64;
    let eth_supply = pooled_eth._0.to_string();

    let block = ctx.block(false).await.unwrap().unwrap();
    let block_timestamp = block.header.timestamp as i64;

    let result = sqlx::query!(
        r#"INSERT INTO "CbETHBase" (block_number, block_timestamp, eth)
           VALUES ($1, $2, $3)
           ON CONFLICT (block_number) DO NOTHING"#,
        block_number,
        block_timestamp,
        eth_supply,
    )
    .execute(db)
    .await;

    match result {
        Ok(_) => {
            info!(
                "Successfully indexed CbETHBase data for block {}",
                block_number
            );
        }
        Err(e) => {
            error!(
                "Error indexing CbETHBase data for block {}: {:?}",
                block_number, e
            );
        }
    }
}
