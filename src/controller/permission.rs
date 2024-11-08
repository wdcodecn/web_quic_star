use chrono::{DateTime, Utc};
use derive_builder::WebApiGen;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Debug,
    Selectable,
    Serialize,
    Deserialize,
    JsonSchema,
    Default,
    AsChangeset,
    Insertable,
)]
#[diesel(table_name = crate::schema::permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPermission {
    pub name: String,
    pub remark: Option<String>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub is_delete: bool,
}

#[derive(
    Queryable,
    Debug,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Selectable,
    WebApiGen,
    Serialize,
    Deserialize,
    JsonSchema,
    Default,
)]
#[diesel(table_name = crate::schema::permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Permission {
    pub id: i64,
    pub name: String,
    pub remark: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub update_by: Option<i64>,
    pub is_delete: bool,
}
pub fn web_routes2(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
    let router_add = ApiRouter::new().api_route(
        "/create_entity",
        post_with(web::create_entity, empty_resp_docs),
    );
    let router_read = ApiRouter::new()
        .api_route(
            "/get_entity_by_id/:id",
            get_with(web::get_entity_by_id, default_resp_docs::<Permission>),
        )
        .api_route(
            "/get_entity_page",
            post_with(web::get_entity_page, empty_resp_docs),
        );
    let router_update = ApiRouter::new().api_route(
        "/update_entity_by_id/:id",
        put_with(web::update_entity_by_id, default_resp_docs::<Permission>),
    );
    let router_delete = ApiRouter::new().api_route(
        "/delete_entity_by_id/:id",
        delete_with(web::delete_entity_by_id, default_resp_docs::<Permission>),
    );
    router_add
        .route_layer(permission_required!(AuthBackend, "users_add"))
        .merge(router_read.route_layer(permission_required!(AuthBackend, "users_read")))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
        .merge(router_update.route_layer(permission_required!(AuthBackend, "users_update")))
        .with_state(conn_pool)
}
