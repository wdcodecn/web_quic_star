use diesel::r2d2::ConnectionManager;
use std::str::FromStr;

use r2d2::Pool;

use crate::AppRes;
use alloy_chains::Chain;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::debug;
//
// #[derive(Debug)]
// pub struct Event {
//     pub signature: String,
//     pub slot: u64,
//     pub user: Pubkey,
//     pub token: Pubkey,
//     pub sol_amount: u64,
//     pub token_amount: u64,
//     pub event_type: EventType, // 新增的字段 buy/sell
// }
//
// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
// pub enum EventType {
//     Buy,
//     Sell,
// }

pub async fn subscribe_uniswap_v2_transaction(
    chain: Chain,
    pool: Pool<ConnectionManager<PgConnection>>,
) -> AppRes<()> {
    loop {
        debug!("subscribe {:?} uniswap v2 transaction", chain);
        // tokio sleep 10 s
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
    Ok(())
}

pub async fn subscribe_uniswap_v3_transaction(
    chain: Chain,
    pool: Pool<ConnectionManager<PgConnection>>,
) -> AppRes<()> {
    loop {
        debug!("subscribe {:?} uniswap v3 transaction", chain);
        // tokio sleep 10 s
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
    Ok(())
}
//
// pub async fn handle_transaction(pool: ConnPool, event: Event) -> AppRes<Swap> {
//     let token = event.token;
//
//     let dex_pool = dex_pools
//         .filter(token_addr.eq(token.to_string()))
//         .select(DexPool::as_select())
//         .first(&mut pool.get()?)
//         .optional()?;
//
//     match dex_pool {
//         Some(x) => {
//             let is_buy = event.event_type == EventType::Buy;
//
//             let curve_token_amount;
//             let curve_sol_amount;
//             if is_buy {
//                 curve_sol_amount = x.pool_curve_sol.add(BigDecimal::from(event.sol_amount));
//                 curve_token_amount = x.pool_curve_token.sub(BigDecimal::from(event.token_amount));
//             } else {
//                 curve_sol_amount = x.pool_curve_sol.sub(BigDecimal::from(event.sol_amount));
//                 curve_token_amount = x.pool_curve_token.add(BigDecimal::from(event.token_amount));
//             }
//
//             let new_swap = NewSwap {
//                 create_time: SystemTime::now().into(),
//                 create_by: SUPER_USER,
//                 is_delete: false,
//                 pool_id: x.id,
//                 is_buy,
//                 tx_hash: event.signature,
//                 sol_amount: event.sol_amount.into(),
//                 token_amount: event.token_amount.into(),
//                 swapper: event.user.into(),
//                 curve_token_amount,
//                 curve_sol_amount,
//             };
//
//             let new_swap1 = new_swap.clone();
//
//             let swap_insert: Swap = diesel::insert_into(swaps)
//                 .values(new_swap)
//                 .returning(Swap::as_returning())
//                 .get_result(&mut pool.get()?)?;
//
//             //  更新pool curve sol 数量
//
//             let sol_amount = event.sol_amount;
//             let token_amount = event.token_amount;
//
//             update_dex_pool_curve(
//                 pool.clone(),
//                 new_swap1.pool_id,
//                 sol_amount as i64,
//                 token_amount as i64,
//                 new_swap1.is_buy,
//             )
//             .await?;
//             Ok(swap_insert)
//         }
//         None => Err(AppError::new(format!(
//             "dex pool with token: {} not found",
//             token.clone().to_string()
//         ))),
//     }
// }
