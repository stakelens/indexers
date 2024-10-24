use crate::db;

use ghost_crab::prelude::*;
use log::{error, info};

const BEACON_DEPOSIT: Address = address!("00000000219ab540356cBB839Cbe05303d7705Fa");

#[block_handler(BeaconDeposit)]
async fn BeaconDeposit(ctx: BlockContext) {
    let db = db::get().await;
    let block_number = ctx.block_number as i64;
    let block = ctx.block(false).await.unwrap().unwrap();
    let block_timestamp = block.header.timestamp as i64;

    let balance = ctx.provider.get_balance(BEACON_DEPOSIT).await.unwrap();
    let eth = balance.to_string();

    let result = sqlx::query!(
        r#"INSERT INTO "BeaconDeposit" (block_number, block_timestamp, eth)
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
                "Successfully saved BeaconDeposit data for block {}",
                block_number
            );
        }
        Err(e) => {
            error!(
                "Error saving BeaconDeposit data for block {}: {:?}",
                block_number, e
            );
        }
    }
}
