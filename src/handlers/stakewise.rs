use ghost_crab::prelude::*;

use crate::db;

#[template(ETHVault.Deposited)]
async fn ETHVaultDeposited(ctx: EventContext) {
    let block_number = ctx.log.block_number.unwrap() as i64;
    let log_index = ctx.log.log_index.unwrap() as i64;
    let eth = event.assets.to_string();

    let block = ctx.block().await.unwrap().unwrap();

    let block_timestamp = block.header.timestamp as i64;

    let db = db::get().await;

    sqlx::query!(
        r#"insert into "StakeWise" (block_number, block_timestamp, log_index, eth) values ($1,$2,$3,$4)"#,
        block_number,
        block_timestamp,
        log_index,
        eth
    )
    .execute(db)
    .await.unwrap();
}

#[template(ETHVault.Redeemed)]
async fn ETHVaultRedeemed(ctx: EventContext) {
    let block_number = ctx.log.block_number.unwrap() as i64;
    let log_index = ctx.log.log_index.unwrap() as i64;
    let eth = format!("-{}", event.assets.to_string());

    let block = ctx.block().await.unwrap().unwrap();

    let block_timestamp = block.header.timestamp as i64;

    let db = db::get().await;

    sqlx::query!(
        r#"insert into "StakeWise" (block_number, block_timestamp, log_index, eth) values ($1,$2,$3,$4)"#,
        block_number,
        block_timestamp,
        log_index,
        eth
    )
    .execute(db)
    .await.unwrap();
}

#[template(ETHVault.Migrated)]
async fn ETHVaultMigrated(ctx: EventContext) {
    let block_number = ctx.log.block_number.unwrap() as i64;
    let log_index = ctx.log.log_index.unwrap() as i64;
    let eth = event.assets.to_string();

    let block = ctx.block().await.unwrap().unwrap();

    let block_timestamp = block.header.timestamp as i64;

    let db = db::get().await;

    sqlx::query!(
        r#"insert into "StakeWise" (block_number, block_timestamp, log_index, eth) values ($1,$2,$3,$4)"#,
        block_number,
        block_timestamp,
        log_index,
        eth
    )
    .execute(db)
    .await.unwrap();
}

#[event_handler(VaultsRegistry.VaultAdded)]
async fn VaultsRegistry(ctx: EventContext) {
    ctx.templates
        .start(Template {
            address: event.vault.clone(),
            start_block: ctx.log.block_number.unwrap(),
            handler: ETHVaultDeposited::new(),
        })
        .await
        .unwrap();

    ctx.templates
        .start(Template {
            address: event.vault.clone(),
            start_block: ctx.log.block_number.unwrap(),
            handler: ETHVaultRedeemed::new(),
        })
        .await
        .unwrap();

    ctx.templates
        .start(Template {
            address: event.vault,
            start_block: ctx.log.block_number.unwrap(),
            handler: ETHVaultMigrated::new(),
        })
        .await
        .unwrap();
}
