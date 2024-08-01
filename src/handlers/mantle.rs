use alloy::{eips::BlockId, sol};
use ghost_crab::prelude::*;
use log::{error, info};

use crate::db;

sol!(
    #[sol(rpc)]
    Staking,
    "abis/mantle/Staking.json"
);

sol!(
    #[sol(rpc)]
    METH,
    "abis/mantle/METH.json"
);

const METH: Address = address!("d5F7838F5C461fefF7FE49ea5ebaF7728bB0ADfa");

#[block_handler(Mantle)]
async fn MantleBlockHandler(ctx: BlockContext) {
    let meth_contract = METH::new(METH, &ctx.provider);

    let METH::stakingContractReturn { _0 } = meth_contract
        .stakingContract()
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    let staking_contract = Staking::new(_0, &ctx.provider);

    let Staking::totalControlledReturn { _0 } = staking_contract
        .totalControlled()
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    let db = db::get().await;

    let block_number = ctx.block_number as i64;
    let eth = _0.to_string();
    let block = ctx.block().await.unwrap().unwrap();
    let block_timestamp = block.header.timestamp as i64;

    let result = sqlx::query!(
        r#"INSERT INTO "Mantle" (block_number, block_timestamp, eth)
           VALUES ($1, $2, $3)
           ON CONFLICT (block_number) DO NOTHING"#,
        block_number,
        block_timestamp,
        eth
    )
    .execute(db)
    .await;

    match result {
        Ok(_) => {
            info!(
                "Successfully indexed Mantle data for block {}",
                block_number
            );
        }
        Err(e) => {
            error!(
                "Error indexing Mantle data for block {}: {:?}",
                block_number, e
            );
        }
    }
}
