use aide::axum::ApiRouter;
use axum_login::permission_required;

use crate::api_auth::login_impl::AuthBackend;
use crate::db_models::group::web::get_routers;
use crate::db_models::ConnPool;

pub fn web_routes2(conn_pool: ConnPool) -> ApiRouter {
    let (router_add, router_read, router_update, router_delete) = get_routers();
    router_add
        .route_layer(permission_required!(AuthBackend, "users_add"))
        .merge(router_read.route_layer(permission_required!(AuthBackend, "users_read")))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
        .merge(router_update.route_layer(permission_required!(AuthBackend, "users_update")))
        .with_state(conn_pool)
}
