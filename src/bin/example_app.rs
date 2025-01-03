use std::env;

use aide::axum::ApiRouter;
use alloy_chains::Chain;
use http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;
use web_quick::framework::api_doc::{fallback, set_api_doc};
use web_quick::framework::auth::get_auth_layer;
use web_quick::framework::db::setup_connection_pool;
use web_quick::scheduled_task::set_scheduler;
use web_quick::{set_env, FILE_SERVER_DIRECTORY};
use web_quick::subscribe::transaction::{subscribe_uniswap_v2_transaction, subscribe_uniswap_v3_transaction};
use web_quick::utils::subscribe::{subscribe_with_retry, subscribe_with_retry_with_chain};

#[tokio::main]
async fn main() {
    web_quick::set_log();
    set_env();

    let connection_pool = setup_connection_pool();
    set_scheduler(connection_pool.clone()).await;

    aide::gen::extract_schemas(true);

    let app = ApiRouter::new()
        .nest_api_service("/auth", web_quick::api::auth::router())
        .nest_api_service(
            "/users",
            web_quick::api::user::user_routes(connection_pool.clone()),
        )
        .nest_api_service(
            "/user_with_group",
            web_quick::db_models::user_with_group_views::web_routes(connection_pool.clone()),
        )
        .nest_api_service(
            "/groups",
            web_quick::api::group::group_router(connection_pool.clone()),
        )
        .nest_api_service("/upload", web_quick::api::upload::upload_routes())
        .nest_service(FILE_SERVER_DIRECTORY, ServeDir::new("assets"))
        .nest_api_service(
            "/permissions",
            web_quick::api::permission::permission_routes(connection_pool.clone()),
        )
        .nest_api_service(
            "/group_permission",
            web_quick::api::group_permission::group_permission_routes(connection_pool.clone()),
        )
        .fallback(fallback)
        .with_state(connection_pool.clone())
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        .layer(get_auth_layer(connection_pool.clone()));



    tokio::spawn(subscribe_with_retry_with_chain(
        connection_pool.clone(),
        Chain::mainnet(),
        subscribe_uniswap_v2_transaction,
    ));
    tokio::spawn(subscribe_with_retry_with_chain(
        connection_pool.clone(),
        Chain::mainnet(),
        subscribe_uniswap_v3_transaction,
    ));

    tokio::spawn(subscribe_with_retry_with_chain(
        connection_pool.clone(),
        Chain::bsc_mainnet(),
        subscribe_uniswap_v2_transaction,
    ));
    tokio::spawn(subscribe_with_retry_with_chain(
        connection_pool.clone(),
        Chain::bsc_mainnet(),
        subscribe_uniswap_v3_transaction,
    ));

    tokio::spawn(subscribe_with_retry_with_chain(
        connection_pool.clone(),
        Chain::base_mainnet(),
        subscribe_uniswap_v2_transaction,
    ));
    tokio::spawn(subscribe_with_retry_with_chain(
        connection_pool.clone(),
        Chain::base_mainnet(),
        subscribe_uniswap_v3_transaction,
    ));


    let doc_app = set_api_doc(app);
    let server_port = env::var("SERVER_PORT").unwrap_or("5090".to_string());
    #[cfg(feature = "dev")]
    info!(
        "{}",
        format!("Api docs are accessible at http://127.0.0.1:{server_port}/docs")
    );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", server_port))
        .await
        .expect("Can not bind to port");
    axum::serve(listener, doc_app)
        .await
        .expect("Can not run server");
}
