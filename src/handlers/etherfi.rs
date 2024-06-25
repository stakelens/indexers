use ghost_crab::prelude::*;

use crate::db;

#[handler(EtherFi.TVLUpdated)]
async fn EtherFiTVLUpdated(ctx: Context) {
    let block_number = ctx.log.block_number.unwrap() as i64;
    let current_tvl = event._currentTvl.to::<i64>();
    let log_index = ctx.log.log_index.unwrap() as i64;

    let db = db::get().await;

    sqlx::query!(
        r#"insert into "EtherFi" (block_number, log_index, tvl) values ($1, $2, $3)"#,
        block_number,
        log_index,
        current_tvl
    )
    .execute(db)
    .await
    .unwrap();
}
