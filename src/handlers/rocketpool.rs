use alloy::{eips::BlockId, primitives::Uint, sol};
use ghost_crab::prelude::*;

use crate::db;

sol!(
    #[sol(rpc)]
    RocketMinipoolManager,
    "abis/rocketpool/RocketMinipoolManager.json"
);

sol!(
    #[sol(rpc)]
    RocketNodeStaking,
    "abis/rocketpool/RocketNodeStaking.json"
);

sol!(
    #[sol(rpc)]
    RocketVault,
    "abis/rocketpool/RocketVault.json"
);

const ROCKET_VAULT: Address = address!("3bdc69c4e5e13e52a65f5583c23efb9636b469d6");
const ROCKET_MINIPOOL: Address = address!("6d010c43d4e96d74c422f2e27370af48711b49bf");
const ROCKET_NODE_STAKING: Address = address!("0d8d8f8541b12a0e1194b7cc4b6d954b90ab82ec");

#[block_handler(RocketPool)]
async fn RocketPoolBlockHandler(ctx: BlockContext) {
    let rocket_vault_contract = RocketVault::new(ROCKET_VAULT, &ctx.provider);
    let rocket_minipool_contract = RocketMinipoolManager::new(ROCKET_MINIPOOL, &ctx.provider);
    let rocket_node_staking_contract = RocketNodeStaking::new(ROCKET_NODE_STAKING, &ctx.provider);

    let mut total_eth: Uint<256, 4> = Uint::from(0);
    let mut total_rpl: Uint<256, 4> = Uint::from(0);

    let limit = 400;
    let mut offset = 0;

    let mut initialised_minipools: Uint<256, 4> = Uint::from(0);
    let mut prelaunch_minipools: Uint<256, 4> = Uint::from(0);
    let mut staking_minipools: Uint<256, 4> = Uint::from(0);
    let mut withdrawable_minipools: Uint<256, 4> = Uint::from(0);

    loop {
        let active_minipools = rocket_minipool_contract
            .getMinipoolCountPerStatus(Uint::from(offset), Uint::from(limit))
            .block(BlockId::from(ctx.block_number))
            .call()
            .await
            .unwrap();

        initialised_minipools += active_minipools.initialisedCount;
        prelaunch_minipools += active_minipools.prelaunchCount;
        staking_minipools += active_minipools.stakingCount;
        withdrawable_minipools += active_minipools.withdrawableCount;

        let mut total: u64 = 0;

        total += active_minipools.initialisedCount.to::<u64>();
        total += active_minipools.prelaunchCount.to::<u64>();
        total += active_minipools.stakingCount.to::<u64>();
        total += active_minipools.withdrawableCount.to::<u64>();
        total += active_minipools.dissolvedCount.to::<u64>();

        if total < limit {
            break;
        }

        offset += limit;
    }

    let mut eth_locked_in_minipools: Uint<256, 4> = Uint::from(0);

    eth_locked_in_minipools += initialised_minipools * Uint::from(16);
    eth_locked_in_minipools += prelaunch_minipools * Uint::from(32);
    eth_locked_in_minipools += staking_minipools * Uint::from(32);
    eth_locked_in_minipools += withdrawable_minipools * Uint::from(32);
    eth_locked_in_minipools = eth_locked_in_minipools * Uint::from(1e18);

    total_eth += eth_locked_in_minipools;

    let rocket_deposit_pool_eth = rocket_vault_contract
        .balanceOf(String::from("rocketDepositPool"))
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    total_eth += rocket_deposit_pool_eth._0;

    let total_rpl_stacked = rocket_node_staking_contract
        .getTotalRPLStake()
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    total_rpl += total_rpl_stacked._0;

    let rocket_dao_node_trusted_actions_rpl_balance = rocket_vault_contract
        .balanceOf(String::from("rocketDAONodeTrustedActions"))
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    total_rpl += rocket_dao_node_trusted_actions_rpl_balance._0;

    let rocket_auction_manager_rpl_balance = rocket_vault_contract
        .balanceOf(String::from("rocketAuctionManager"))
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    total_rpl += rocket_auction_manager_rpl_balance._0;

    let db = db::get().await;

    let block_number = ctx.block_number as i64;
    let total_eth = total_eth.to_string();
    let total_rpl = total_rpl.to_string();
    let block = ctx.block().await.unwrap().unwrap();
    let block_timestamp = block.header.timestamp as i64;

    sqlx::query!(
        r#"insert into "RocketPool" (block_number, block_timestamp, eth, rpl) values ($1,$2,$3,$4)"#,
        block_number,
        block_timestamp,
        total_eth,
        total_rpl,
    )
    .execute(db)
    .await
    .unwrap();
}
