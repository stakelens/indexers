use crate::db;
use ghost_crab::prelude::*;
use log::{error, info};

#[event_handler(EtherFi.TVLUpdated)]
async fn EtherFiTVLUpdated(ctx: EventContext) {
    let block_number = ctx.log.block_number.unwrap() as i64;
    let current_tvl = event._currentTvl.to_string();
    let log_index = ctx.log.log_index.unwrap() as i64;

    let block = ctx.block().await.unwrap().unwrap();

    let block_timestamp = block.header.timestamp as i64;
    let db = db::get().await;

    let result = sqlx::query!(
        r#"INSERT INTO "EtherFi" (block_number, block_timestamp, log_index, eth)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (block_number, log_index) DO NOTHING"#,
        block_number,
        block_timestamp,
        log_index,
        current_tvl
    )
    .execute(db)
    .await;

    match result {
        Ok(_) => {
            info!(
                "Successfully indexed EtherFi TVL data for block {}, log index {}",
                block_number, log_index
            );
        }
        Err(e) => {
            error!(
                "Error indexing EtherFi TVL data for block {}, log index {}: {:?}",
                block_number, log_index, e
            );
        }
    }
}
