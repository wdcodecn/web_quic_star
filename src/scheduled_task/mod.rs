use crate::db_models::{Conn, ConnPool};
use crate::AppRes;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use std::future::Future;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{debug, error, info};
pub async fn add_async_cron<R>(
    sched: &JobScheduler,
    pool: ConnPool,
    cron: &str,
    task: fn(PooledConnection<ConnectionManager<Conn>>) -> R,
) where
    R: Future<Output = AppRes<()>> + Sized + Send + 'static,
{
    sched
        .add(
            Job::new_async(cron, move |_uuid, _l| {
                let conn = pool.get().expect("cannot get pool");
                Box::pin(async move {
                    match task(conn).await {
                        Ok(_) => {
                            debug!("cron task succeed");
                        }
                        Err(e) => {
                            error!("cron task failed: {}", e);
                        }
                    };
                })
            })
            .expect("cannot create job"),
        )
        .await
        .expect("cannot join job");
}
pub async fn example(mut conn: PooledConnection<ConnectionManager<Conn>>) -> AppRes<()> {
    Ok(())
}

pub async fn set_scheduler(pool: ConnPool) {
    let sched = JobScheduler::new()
        .await
        .expect("cannot create jobs scheduler");
    add_async_cron(&sched, pool.clone(), "1 1/10 * * * *", example).await;

    sched.start().await.expect("cannot start jobs scheduler");
}
