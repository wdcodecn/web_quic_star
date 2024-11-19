use crate::api;
use crate::api::auth;
use crate::api::group_permission::web;
use crate::db_models::group::web::get_routers;
use crate::db_models::group_permission::GroupsPermission;
use crate::db_models::user::User;
use crate::db_models::ConnPool;
use crate::db_models::{permission, user};
use crate::framework::api_doc::{default_resp_docs, empty_resp_docs};
use crate::framework::auth::AuthBackend;
use aide::axum::routing::{delete_with, get_with, post_with};
use aide::axum::ApiRouter;
use axum_login::{login_required, permission_required};

pub fn router() -> ApiRouter<()> {
    ApiRouter::new().api_route("/login", post_with(auth::login, default_resp_docs::<User>))
}

pub fn group_router(conn_pool: ConnPool) -> ApiRouter {
    let (router_add, router_read, router_update, router_delete) = get_routers();
    router_add
        .route_layer(permission_required!(AuthBackend, "users_add"))
        .merge(router_read.route_layer(permission_required!(AuthBackend, "users_read")))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
        .merge(router_update.route_layer(permission_required!(AuthBackend, "users_update")))
        .with_state(conn_pool)
}

pub fn group_permission_routes(conn_pool: ConnPool) -> ApiRouter {
    let router_add = ApiRouter::new().api_route(
        "/create_entity",
        post_with(
            crate::api::group_permission::web::create_entity,
            empty_resp_docs,
        ),
    );
    let router_read = ApiRouter::new()
        .api_route(
            "/get_entity_by_id/:group_id/:permission_id",
            get_with(web::get_entity_by_id, default_resp_docs::<GroupsPermission>),
        )
        .api_route(
            "/get_entity_page",
            post_with(web::get_entity_page, empty_resp_docs),
        );
    let router_delete = ApiRouter::new().api_route(
        "/delete_entity_by_id/:group_id/:permission_id",
        delete_with(
            web::delete_entity_by_id,
            default_resp_docs::<GroupsPermission>,
        ),
    );
    router_add
        .route_layer(permission_required!(AuthBackend, "users_add"))
        .merge(router_read.route_layer(permission_required!(AuthBackend, "users_read")))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
        .with_state(conn_pool)
}

pub fn permission_routes(conn_pool: ConnPool) -> ApiRouter {
    let (router_add, router_read, router_update, router_delete) = permission::web::get_routers();

    router_add
        .route_layer(permission_required!(AuthBackend, "users_add"))
        .merge(router_read.route_layer(permission_required!(AuthBackend, "users_read")))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
        .merge(router_update.route_layer(permission_required!(AuthBackend, "users_update")))
        .with_state(conn_pool)
}

pub fn user_routes(conn_pool: ConnPool) -> ApiRouter {
    let (router_add, router_read, router_update, router_delete) = user::web::get_routers();
    let modify_password = ApiRouter::new().api_route(
        "/modify_password",
        post_with(api::user::modify_password, default_resp_docs::<String>),
    );
    router_add
        .route_layer(permission_required!(AuthBackend, "users_add"))
        .merge(router_read.route_layer(permission_required!(AuthBackend, "users_read")))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
        .merge(router_update.route_layer(permission_required!(AuthBackend, "users_update")))
        .merge(modify_password.route_layer(login_required!(AuthBackend)))
        .with_state(conn_pool)
}
