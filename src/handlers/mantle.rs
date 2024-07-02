use alloy::{rpc::types::eth::BlockNumberOrTag, sol};
use ghost_crab::prelude::*;

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

#[block_handler(Mantle)]
async fn MantleBlockHandler(ctx: BlockContext) {
    let block_number = ctx.block.header.number.unwrap();

    let meth_contract = METH::new(
        "0xd5F7838F5C461fefF7FE49ea5ebaF7728bB0ADfa"
            .parse()
            .unwrap(),
        &ctx.provider,
    );

    let METH::stakingContractReturn { _0 } = meth_contract
        .stakingContract()
        .block(alloy::rpc::types::eth::BlockId::Number(
            BlockNumberOrTag::Number(block_number),
        ))
        .call()
        .await
        .unwrap();

    let staking_contract = Staking::new(_0, &ctx.provider);

    let Staking::totalControlledReturn { _0 } = staking_contract
        .totalControlled()
        .block(alloy::rpc::types::eth::BlockId::Number(
            BlockNumberOrTag::Number(block_number),
        ))
        .call()
        .await
        .unwrap();

    let db = db::get().await;

    let block_number = block_number as i64;
    let eth = _0.to_string();
    let block_timestamp = ctx.block.header.timestamp as i64;

    sqlx::query!(
        r#"insert into "Mantle" (block_number, block_timestamp, eth) values ($1,$2,$3)"#,
        block_number,
        block_timestamp,
        eth
    )
    .execute(db)
    .await
    .unwrap();
}
