#![feature(backtrace_frames)]
#![feature(negative_impls)]
use crate::api_auth::login_impl::AuthBackend;
use axum::{Extension, Router};
use std::env;
use std::sync::{Arc, OnceLock};

use crate::api_doc::api_docs;
use crate::api_doc::docs::docs_routes;
use crate::api_doc::errors::AppError;
use aide::axum::ApiRouter;
use aide::openapi::OpenApi;
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use tokio_cron_scheduler::{Job, JobScheduler};

pub mod api_auth;
pub mod api_doc;
pub mod api_wrapper;
#[cfg(feature = "eth_mode")]
pub mod contracts;
pub mod controller;
pub mod domain;
pub mod models;
pub mod scheduled_task;
pub mod schema;

type AppRes<T> = Result<T, AppError>;

pub static GLOBAL_CONNECTION_POOL: OnceLock<Pool<ConnectionManager<PgConnection>>> =
    OnceLock::new();

pub async fn set_scheduler() {
    let sched = JobScheduler::new()
        .await
        .expect("cannot create jobs scheduler");
    sched
        .add(
            Job::new("1/10 * * * * *", |_uuid, _l| {
                // println!("I run every 10 seconds");
            })
            .expect("cannot create job"),
        )
        .await
        .expect("cannot join job");

    sched.start().await.expect("cannot start jobs scheduler");
}

pub fn set_api_doc(app: ApiRouter) -> Router {
    aide::gen::on_error(|error| {
        println!("{error}");
    });
    aide::gen::extract_schemas(true);
    let mut api = OpenApi::default();
    app.nest_api_service("/docs", docs_routes())
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)))
}

pub fn get_auth_layer(
    connection_pool: Pool<ConnectionManager<PgConnection>>,
) -> AuthManagerLayer<AuthBackend, MemoryStore> {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let backend = AuthBackend::new(connection_pool);
    AuthManagerLayerBuilder::new(backend, session_layer).build()
}

pub fn set_env() {
    #[cfg(feature = "dev")]
    {
        tracing::info!("profile :{} is active", "dev");
        dotenvy::from_filename(".env").expect("no .env file");
    }
    #[cfg(not(feature = "dev"))]
    {
        tracing::info!("profile :{} is active", "release");
        dotenvy::from_filename("env_prod.env").expect("no env_prod.env file");
    }
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    let connection_pool = Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");
    GLOBAL_CONNECTION_POOL.get_or_init(|| connection_pool.clone());
    connection_pool
}

pub fn set_log() {
    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                // .with_file(true)
                .with_line_number(true),
        )
        .init();
}
