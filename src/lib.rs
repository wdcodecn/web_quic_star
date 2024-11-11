#![feature(backtrace_frames)]
#![feature(negative_impls)]

use crate::api_doc::errors::AppError;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;
use tracing::error;

pub mod api_auth;
pub mod api_doc;
pub mod api_wrapper;
#[cfg(feature = "eth_mode")]
pub mod contracts;

#[cfg(feature = "postgres")]
pub mod controller;
#[cfg(feature = "postgres")]
pub mod db_models;
pub mod domain;
pub mod scheduled_task;
pub mod schema;

type AppRes<T> = Result<T, AppError>;

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
    Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn set_log() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .event_format(
            tracing_subscriber::fmt::format()
                // .with_file(true)
                .with_line_number(true),
        )
        .init();
    aide::gen::on_error(|error| {
        error!("{error}");
    });
}
