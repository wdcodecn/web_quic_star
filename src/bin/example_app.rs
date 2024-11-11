use std::env;

use aide::axum::ApiRouter;
use tracing::{error, info};
use web_quic_star::api_auth::get_auth_layer;
use web_quic_star::api_doc::{fallback, set_api_doc};
use web_quic_star::scheduled_task::set_scheduler;
use web_quic_star::{api_auth, controller, get_connection_pool, set_env};

#[tokio::main]
async fn main() {
    web_quic_star::set_log();
    set_env();

    let connection_pool = get_connection_pool();
    set_scheduler(connection_pool.clone()).await;
    aide::gen::on_error(|error| {
        error!("{error}");
    });
    aide::gen::extract_schemas(true);

    let app = ApiRouter::new()
        .nest_api_service("/auth", api_auth::router::router())
        .nest_api_service(
            "/users",
            controller::user::web_routes2(connection_pool.clone()),
        )
        .nest_api_service(
            "/groups",
            controller::group::web_routes2(connection_pool.clone()),
        )
        .nest_api_service(
            "/permissions",
            controller::permission::web_routes2(connection_pool.clone()),
        )
        .nest_api_service(
            "/group_permission",
            controller::group_permission::web_routes(connection_pool.clone()),
        )
        .fallback(fallback)
        .with_state(connection_pool.clone())
        .layer(get_auth_layer(connection_pool.clone()));

    let doc_app = set_api_doc(app);
    let server_port = env::var("SERVER_PORT").unwrap_or("4090".to_string());
    #[cfg(feature = "dev")]
    info!(
        "{}",
        format!("Api docs are accessible at http://127.0.0.1:{server_port}/docs")
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", server_port))
        .await
        .expect("Can not bind to port");
    axum::serve(listener, doc_app)
        .await
        .expect("Can not run server");
}
