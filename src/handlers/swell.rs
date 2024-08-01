use alloy::{eips::BlockId, primitives::Uint, sol};
use ghost_crab::prelude::*;
use log::{error, info};

use crate::db;

sol!(
    #[sol(rpc)]
    swETH,
    "abis/swell/swETH.json"
);

const SWETH: Address = address!("f951E335afb289353dc249e82926178EaC7DEd78");

#[block_handler(Swell)]
async fn SwellBlockHandler(ctx: BlockContext) {
    let sweth_contract = swETH::new(SWETH, &ctx.provider);

    let total_supply = sweth_contract
        .totalSupply()
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    let rate = sweth_contract
        .getRate()
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    let db = db::get().await;

    let block_number = ctx.block_number as i64;
    let eth_supply = ((total_supply._0 * rate._0) / Uint::from(1e18)).to_string();
    let block = ctx.block().await.unwrap().unwrap();
    let block_timestamp = block.header.timestamp as i64;

    let result = sqlx::query!(
        r#"INSERT INTO "Swell" (block_number, block_timestamp, eth) 
           VALUES ($1, $2, $3)
           ON CONFLICT (block_number) 
           DO NOTHING"#,
        block_number,
        block_timestamp,
        eth_supply,
    )
    .execute(db)
    .await;

    match result {
        Ok(_) => {
            info!("Successfully saved indexed data for block {}", block_number);
        }
        Err(e) => {
            error!("Error indexing data for block {}: {:?}", block_number, e);
        }
    }
}
