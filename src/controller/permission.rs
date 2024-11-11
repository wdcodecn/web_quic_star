use aide::axum::ApiRouter;
use axum_login::permission_required;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

use crate::api_auth::login_impl::AuthBackend;
use crate::db_models::permission::web::get_routers;

pub fn web_routes2(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
    let (router_add, router_read, router_update, router_delete) = get_routers();

    router_add
        .route_layer(permission_required!(AuthBackend, "users_add"))
        .merge(router_read.route_layer(permission_required!(AuthBackend, "users_read")))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
        .merge(router_update.route_layer(permission_required!(AuthBackend, "users_update")))
        .with_state(conn_pool)
}
