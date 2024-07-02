use alloy::{primitives::address, rpc::types::eth::BlockNumberOrTag, sol};
use ghost_crab::prelude::*;

use crate::db;

sol!(
    #[sol(rpc)]
    ezETHToken,
    "abis/renzo/EzEthToken.json"
);

#[block_handler(Renzo)]
async fn RenzoBlockHandler(ctx: BlockContext) {
    let block_number = ctx.block.header.number.unwrap();

    let ezeth_contract = ezETHToken::new(
        "0xf951E335afb289353dc249e82926178EaC7DEd78"
            .parse()
            .unwrap(),
        &ctx.provider,
    );

    let total_supply = ezeth_contract
        .totalSupply()
        .block(alloy::rpc::types::eth::BlockId::Number(
            BlockNumberOrTag::Number(block_number),
        ))
        .call()
        .await
        .unwrap();

    let lock_box_balance = ezeth_contract
        .balanceOf(address!("C8140dA31E6bCa19b287cC35531c2212763C2059"))
        .block(alloy::rpc::types::eth::BlockId::Number(
            BlockNumberOrTag::Number(block_number),
        ))
        .call()
        .await
        .unwrap();

    let eth = total_supply._0 - lock_box_balance._0;

    let db = db::get().await;

    let block_number = block_number as i64;
    let eth = eth.to_string();
    let block_timestamp = ctx.block.header.timestamp as i64;

    sqlx::query!(
        r#"insert into "Renzo" (block_number, block_timestamp, eth) values ($1,$2,$3)"#,
        block_number,
        block_timestamp,
        eth,
    )
    .execute(db)
    .await
    .unwrap();
}
