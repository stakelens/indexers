use ghost_crab::prelude::*;

use crate::db;

#[handler(ETHVault.Deposited)]
async fn ETHVaultDeposited(ctx: Context) {
    let decoded_log = ctx
        .log
        .log_decode::<ETHVaultDepositedContract::Deposited>()
        .unwrap();

    let data = decoded_log.data();

    let block_number = ctx.log.block_number.unwrap().to_string();
    let log_index = ctx.log.log_index.unwrap().to_string();
    let vault = data.receiver.to_string();
    let eth = data.assets.to_string();

    let db = db::get().await;

    sqlx::query!(
        "insert into StakeWise (block_number, log_index, vault, eth) values (?,?,?,?)",
        block_number,
        log_index,
        vault,
        eth
    )
    .execute(db)
    .await
    .unwrap();
}

#[handler(ETHVault.Redeemed)]
async fn ETHVaultRedeemed(ctx: Context) {
    let decoded_log = ctx
        .log
        .log_decode::<ETHVaultRedeemedContract::Redeemed>()
        .unwrap();

    let data = decoded_log.data();

    let block_number = ctx.log.block_number.unwrap().to_string();
    let log_index = ctx.log.log_index.unwrap().to_string();
    let vault = data.owner.to_string();
    let eth = format!("-{}", data.assets.to_string());

    let db = db::get().await;

    sqlx::query!(
        "insert into StakeWise (block_number, log_index, vault, eth) values (?,?,?,?)",
        block_number,
        log_index,
        vault,
        eth
    )
    .execute(db)
    .await
    .unwrap();
}

#[handler(VaultsRegistry.VaultAdded)]
async fn VaultsRegistry(ctx: Context) {
    let decoded_log = ctx
        .log
        .log_decode::<VaultsRegistryContract::VaultAdded>()
        .unwrap();

    let data = decoded_log.data();

    let vault = data.vault.to_string();

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
