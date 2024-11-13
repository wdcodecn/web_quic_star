use crate::AppRes;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};
use std::future::Future;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{debug, error, info};

pub async fn add_async_cron<R>(
    sched: &JobScheduler,
    pool: Pool<ConnectionManager<PgConnection>>,
    cron: &str,
    task: fn(PooledConnection<ConnectionManager<PgConnection>>) -> R,
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
pub async fn example(mut conn: PooledConnection<ConnectionManager<PgConnection>>) -> AppRes<()> {
    Ok(())
}

pub async fn set_scheduler(pool: Pool<ConnectionManager<PgConnection>>) {
    let sched = JobScheduler::new()
        .await
        .expect("cannot create jobs scheduler");
    add_async_cron(&sched, pool.clone(), "1 1/10 * * * *", example).await;

    sched.start().await.expect("cannot start jobs scheduler");
}
