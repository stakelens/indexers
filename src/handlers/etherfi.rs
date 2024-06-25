use ghost_crab::prelude::*;

use crate::db;

#[handler(EtherFi.TVLUpdated)]
async fn EtherFiTVLUpdated(ctx: Context) {
    let block_number = ctx.log.block_number.unwrap().to_string();
    let current_tvl = event._currentTvl.to_string();
    let log_index = ctx.log.log_index.unwrap().to_string();

    let db = db::get().await;

    sqlx::query!(
        "insert into EtherFi (block_number, log_index, tvl) values (?, ?, ?)",
        block_number,
        log_index,
        current_tvl
    )
    .execute(db)
    .await
    .unwrap();
}
