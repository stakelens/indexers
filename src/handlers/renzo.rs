use alloy::{eips::BlockId, primitives::address, sol};
use ghost_crab::prelude::*;
use log::{error, info};

use crate::db;

sol!(
    #[sol(rpc)]
    ezETHToken,
    "abis/renzo/EzEthToken.json"
);

const EZETH: Address = address!("f951E335afb289353dc249e82926178EaC7DEd78");
const LOCK_BOX: Address = address!("C8140dA31E6bCa19b287cC35531c2212763C2059");

#[block_handler(Renzo)]
async fn RenzoBlockHandler(ctx: BlockContext) {
    let ezeth_contract = ezETHToken::new(EZETH, &ctx.provider);

    let block_id = BlockId::from(ctx.block_number);

    let total_supply = ezeth_contract
        .totalSupply()
        .block(block_id.clone())
        .call()
        .await
        .unwrap();

    let lock_box_balance = ezeth_contract
        .balanceOf(LOCK_BOX)
        .block(block_id)
        .call()
        .await
        .unwrap();

    let eth = total_supply._0 - lock_box_balance._0;

    let db = db::get().await;

    let block_number = ctx.block_number as i64;
    let eth = eth.to_string();
    let block = ctx.block().await.unwrap().unwrap();
    let block_timestamp = block.header.timestamp as i64;

    let result = sqlx::query!(
        r#"INSERT INTO "Renzo" (block_number, block_timestamp, eth)
           VALUES ($1, $2, $3)
           ON CONFLICT (block_number) DO NOTHING"#,
        block_number,
        block_timestamp,
        eth,
    )
    .execute(db)
    .await;

    match result {
        Ok(_) => {
            info!("Successfully indexed Renzo data for block {}", block_number);
        }
        Err(e) => {
            error!(
                "Error indexing Renzo data for block {}: {:?}",
                block_number, e
            );
        }
    }
}
