#![feature(backtrace_frames)]
#![feature(negative_impls)]

use crate::api_doc::errors::AppError;
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

pub fn set_log() {
    tracing_subscriber::fmt()
        .pretty()
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
