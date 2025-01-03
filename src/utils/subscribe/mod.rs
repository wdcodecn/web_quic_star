use crate::db_models::ConnPool;
use crate::AppRes;
use std::future::Future;
use std::time::Duration;
use alloy_chains::Chain;
use tokio::time::sleep;
use tracing::error;

pub async fn subscribe_with_retry<F>(pool: ConnPool, func: fn(ConnPool) -> F) -> AppRes<()>
where
    F: Future<Output = AppRes<()>> + Sized,
{
    loop {
        match func(pool.clone()).await {
            Ok(_) => {}
            Err(e) => {
                error!(?e, " Will retry subscribe");
                sleep(Duration::new(2, 0)).await;
            }
        };
    }
}

pub async fn subscribe_with_retry_with_chain<F>(pool: ConnPool,chain: Chain, func: fn(Chain,ConnPool) -> F) -> AppRes<()>
where
    F: Future<Output = AppRes<()>> + Sized,
{
    loop {
        match func(chain.clone(),pool.clone()).await {
            Ok(_) => {}
            Err(e) => {
                error!(?e, " Will retry subscribe");
                sleep(Duration::new(2, 0)).await;
            }
        };
    }
}
