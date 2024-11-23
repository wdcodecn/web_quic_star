use crate::db_models::ConnPool;
use crate::AppRes;
use std::future::Future;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{debug, error};
pub async fn add_async_cron<R>(
    sched: &JobScheduler,
    pool: ConnPool,
    cron: &str,
    task: fn(ConnPool) -> R,
) where
    R: Future<Output = AppRes<()>> + Sized + Send + 'static,
{
    sched
        .add(
            Job::new_async(cron, move |_uuid, _l| {
                let pool = pool.clone();
                Box::pin(async move {
                    match task(pool).await {
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
pub async fn example(mut _conn: ConnPool) -> AppRes<()> {
    Ok(())
}

pub async fn set_scheduler(pool: ConnPool) {
    let sched = JobScheduler::new()
        .await
        .expect("cannot create jobs scheduler");
    add_async_cron(&sched, pool.clone(), "1 1/10 * * * *", example).await;

    sched.start().await.expect("cannot start jobs scheduler");
}
