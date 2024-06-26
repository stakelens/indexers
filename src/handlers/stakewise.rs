use ghost_crab::prelude::*;

use crate::db;

#[handler(ETHVault.Deposited)]
async fn ETHVaultDeposited(ctx: Context) {
    let block_number = ctx.log.block_number.unwrap() as i64;
    let log_index = ctx.log.log_index.unwrap() as i64;
    let vault = event.receiver.to_string();
    let eth = event.assets.to_string();

    let db = db::get().await;

    sqlx::query!(
        r#"insert into "StakeWise" (block_number, log_index, vault, eth) values ($1,$2,$3,$4) ON CONFLICT (block_number, log_index) DO NOTHING"#,
        block_number,
        log_index,
        vault,
        eth
    )
    .execute(db)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.code().as_deref() == Some("23505") {
                return Ok(());
            }
        }
        Err(e)
    }).unwrap();
}

#[handler(ETHVault.Redeemed)]
async fn ETHVaultRedeemed(ctx: Context) {
    let block_number = ctx.log.block_number.unwrap() as i64;
    let log_index = ctx.log.log_index.unwrap() as i64;
    let vault = event.owner.to_string();
    let eth = format!("-{}", event.assets.to_string());

    let db = db::get().await;

    sqlx::query!(
        r#"insert into "StakeWise" (block_number, log_index, vault, eth) values ($1,$2,$3,$4)"#,
        block_number,
        log_index,
        vault,
        eth
    )
    .execute(db)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.code().as_deref() == Some("23505") {
                return Ok(());
            }
        }
        Err(e)
    })
    .unwrap();
}

#[handler(VaultsRegistry.VaultAdded)]
async fn VaultsRegistry(ctx: Context) {
    let vault = event.vault.to_string();

    ctx.templates
        .start(Template {
            address: vault.clone(),
            start_block: ctx.log.block_number.unwrap(),
            handler: ETHVaultDeposited::new(),
        })
        .await;

    ctx.templates
        .start(Template {
            address: vault,
            start_block: ctx.log.block_number.unwrap(),
            handler: ETHVaultRedeemed::new(),
        })
        .await;
}
