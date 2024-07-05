use alloy::{rpc::types::eth::BlockNumberOrTag, sol};
use ghost_crab::prelude::*;

use crate::db;

sol!(
    #[sol(rpc)]
    StaderStakePoolsManager,
    "abis/stader/StaderStakePoolsManager.json"
);

#[block_handler(Stader)]
async fn StaderBlockHandler(ctx: BlockContext) {
    let block_number = ctx.block.header.number.unwrap();

    let stader_staking_manager = StaderStakePoolsManager::new(
        "0xcf5EA1b38380f6aF39068375516Daf40Ed70D299"
            .parse()
            .unwrap(),
        &ctx.provider,
    );

    let total_assets = stader_staking_manager
        .totalAssets()
        .block(alloy::rpc::types::eth::BlockId::Number(
            BlockNumberOrTag::Number(block_number),
        ))
        .call()
        .await
        .unwrap();

    let db = db::get().await;

    let eth = total_assets._0.to_string();
    let block_number = block_number as i64;
    let block_timestamp = ctx.block.header.timestamp as i64;

    sqlx::query!(
        r#"insert into "Stader" (block_number, block_timestamp, eth) values ($1,$2,$3)"#,
        block_number,
        block_timestamp,
        eth,
    )
    .execute(db)
    .await
    .unwrap();
}
