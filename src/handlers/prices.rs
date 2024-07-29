use alloy::sol;
use ghost_crab::prelude::*;
use uniswap_sdk_core::{prelude::*, token};
use uniswap_v3_sdk::prelude::*;
use std::cmp::Ordering;

use crate::db;

sol!(
    #[sol(rpc)]
    UniswapV3Pool,
    "abis/prices/UniswapV3Pool.json"
);

const USDC_ETH_V3: Address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");

struct Observation {
    seconds_ago: u32,
    tick_cumulative: i128,
}

#[block_handler(ETHUSDC)]
async fn PricesBlockHandler(ctx: BlockContext) {
    let uniswap_v3_pool_contract = UniswapV3Pool::new(USDC_ETH_V3, &ctx.provider);

    let timestamps = Vec::from([0, 360]);

    let observe_result = uniswap_v3_pool_contract
        .observe(timestamps.clone())
        .call()
        .await
        .unwrap();

    let tick_cumulatives: Vec<i128> = observe_result.tickCumulatives
        .into_iter()
        .map(|x| x as i128)
        .collect();

    let observations: Vec<Observation> = timestamps
        .iter()
        .enumerate()
        .map(|(i, &seconds_ago)| Observation {
            seconds_ago,
            tick_cumulative: tick_cumulatives[i],
        })
        .collect();

    let diff_tick_cumulative = match observations[0].tick_cumulative.cmp(&observations[1].tick_cumulative) {
        Ordering::Greater => observations[0].tick_cumulative - observations[1].tick_cumulative,
        _ => observations[1].tick_cumulative - observations[0].tick_cumulative,
    };

    let seconds_between = (observations[0].seconds_ago as i128 - observations[1].seconds_ago as i128).abs();
    let average_tick = (diff_tick_cumulative as f64 / seconds_between as f64).round() as i32;

    const CHAIN_ID: u64 = 1; // Ethereum Mainnet
    let usdc: Token = token!(CHAIN_ID, "A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", 6, "USDC", "USD Coin");
    let weth: Token = token!(CHAIN_ID, "C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", 18, "WETH", "Wrapped Ether");

    let price = match tick_to_price(weth, usdc, average_tick) {
        Ok(price_fraction) => {
            let price_decimal = price_fraction.to_decimal();
            match price_decimal.to_f64() {
                Some(p) => p,
                None => {
                    eprintln!("Error: Failed to convert price to f64");
                    return;
                }
            }
        },
        Err(e) => {
            eprintln!("Error calculating price: {:?}", e);
            return;
        }
    };

    let db = db::get().await;

    let block = ctx.block().await.unwrap().unwrap();
    let block_timestamp = block.header.timestamp as i64;

    sqlx::query!(
        r#"insert into "CurrencyPrice" (name, block_timestamp, price) values ($1,$2,$3)"#,
        "ETH/USDC",
        block_timestamp,
        price
    )
    .execute(db)
    .await
    .unwrap();
}