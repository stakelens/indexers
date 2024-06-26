use crate::db;
use alloy::eips::BlockNumberOrTag;
use ghost_crab::prelude::*;

#[handler(EtherFi.TVLUpdated)]
async fn EtherFiTVLUpdated(ctx: Context) {
    let block_number = ctx.log.block_number.unwrap() as i64;
    let current_tvl = event._currentTvl.to_string();
    let log_index = ctx.log.log_index.unwrap() as i64;

    let block = ctx
        .provider
        .get_block_by_number(
            BlockNumberOrTag::Number(ctx.log.block_number.unwrap()),
            false,
        )
        .await
        .unwrap()
        .unwrap();

    let block_timestamp = block.header.timestamp as i64;
    let db = db::get().await;

    sqlx::query!(
        r#"insert into "EtherFi" (block_number, block_timestamp, log_index, tvl) values ($1, $2, $3, $4)"#,
        block_number,
        block_timestamp,
        log_index,
        current_tvl
    )
    .execute(db)
    .await;
}
