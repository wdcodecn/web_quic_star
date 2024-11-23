use std::{env, panic};

use aide::axum::ApiRouter;
use tracing::info;
use web_quick::framework::api_doc::{fallback, set_api_doc};
use web_quick::framework::auth::get_auth_layer;
use web_quick::framework::db::setup_connection_pool;
use web_quick::scheduled_task::set_scheduler;
use web_quick::set_env;

#[tokio::main]
async fn main() {
    web_quick::set_log();
    set_env();

    let connection_pool = setup_connection_pool();
    set_scheduler(connection_pool.clone()).await;

    aide::gen::extract_schemas(true);

    let app = ApiRouter::new()
        .nest_api_service("/auth", web_quick::router::router())
        .nest_api_service(
            "/users",
            web_quick::router::user_routes(connection_pool.clone()),
        )
        .nest_api_service(
            "/groups",
            web_quick::router::group_router(connection_pool.clone()),
        )
        .nest_api_service(
            "/permissions",
            web_quick::router::permission_routes(connection_pool.clone()),
        )
        .nest_api_service(
            "/group_permission",
            web_quick::router::group_permission_routes(connection_pool.clone()),
        )
        .fallback(fallback)
        .with_state(connection_pool.clone())
        .layer(get_auth_layer(connection_pool.clone()));

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
