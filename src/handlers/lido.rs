use alloy::{eips::BlockId, sol};
use ghost_crab::prelude::*;

use crate::db;

sol!(
    #[sol(rpc)]
    Lido,
    "abis/lido/Lido.json"
);

const LIDO: Address = address!("ae7ab96520DE3A18E5e111B5EaAb095312D7fE84");

#[block_handler(Lido)]
async fn LidoBlockHandler(ctx: BlockContext) {
    let lido_contract = Lido::new(LIDO, &ctx.provider);

    let pooled_eth = lido_contract
        .getTotalPooledEther()
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    let db = db::get().await;

    let block_number = ctx.block_number as i64;
    let eth_supply = pooled_eth._0.to_string();

    let block = ctx.block().await.unwrap().unwrap();
    let block_timestamp = block.header.timestamp as i64;

    sqlx::query!(
        r#"insert into "Lido" (block_number, block_timestamp, eth) values ($1,$2,$3)"#,
        block_number,
        block_timestamp,
        eth_supply,
    )
    .execute(db)
    .await
    .unwrap();
}
