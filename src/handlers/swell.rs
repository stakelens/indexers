use alloy::{rpc::types::eth::BlockNumberOrTag, sol};
use ghost_crab::prelude::*;

use crate::db;

sol!(
    #[sol(rpc)]
    swETH,
    "abis/swell/swETH.json"
);

#[block_handler(Swell)]
async fn SwellBlockHandler(ctx: BlockContext) {
    let block_number = ctx.block.header.number.unwrap();

    let sweth_contract = swETH::new(
        "0xf951E335afb289353dc249e82926178EaC7DEd78"
            .parse()
            .unwrap(),
        &ctx.provider,
    );

    let total_supply = sweth_contract
        .totalSupply()
        .block(alloy::rpc::types::eth::BlockId::Number(
            BlockNumberOrTag::Number(block_number),
        ))
        .call()
        .await
        .unwrap();

    let rate = sweth_contract
        .getRate()
        .block(alloy::rpc::types::eth::BlockId::Number(
            BlockNumberOrTag::Number(block_number),
        ))
        .call()
        .await
        .unwrap();

    let db = db::get().await;

    let block_number = block_number as i64;
    let eth_supply = (total_supply._0 * rate._0).to_string();
    let block_timestamp = ctx.block.header.timestamp as i64;

    sqlx::query!(
        r#"insert into "Swell" (block_number, block_timestamp, eth) values ($1,$2,$3)"#,
        block_number,
        block_timestamp,
        eth_supply,
    )
    .execute(db)
    .await
    .unwrap();
}
