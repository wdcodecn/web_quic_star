use framework::api_doc::errors::AppError;
use std::collections::HashMap;
use std::panic;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use tracing::error;
use tracing_subscriber::EnvFilter;

pub mod api;
pub mod constant;
pub mod contract;
pub mod db_models;
pub mod domain;
pub mod framework;
pub mod scheduled_task;
pub mod schema;
pub mod schema_view;
pub mod utils;
pub use contract::*;

type AppRes<T> = Result<T, AppError>;
pub const FILE_SERVER_DIRECTORY: &str = "/assets";
pub type Cache<K, V> = LazyLock<Arc<RwLock<HashMap<K, V>>>>;

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
    panic::set_hook(Box::new(|info| {
        error!(error = %info, "panic occurred");
    }));
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(tracing::Level::TRACE)
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
#[test]
pub fn test() {
    set_log();
    use tracing::error;

    let (err_info, port) = ("No connection", 22);

    error!(err_info);
    error!(target: "app_events", "App Error: {}", err_info);
    error!({ info = err_info }, "error on port: {}", port);
    error!(name: "invalid_input", "Invalid input: {}", err_info);
}
