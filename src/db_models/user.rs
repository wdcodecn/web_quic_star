use crate::api_doc::errors::AppError;
use crate::api_doc::extractors::Json;
use crate::AppRes;
use aide::OperationIo;
use axum::extract::State;
use axum_login::{login_required, AuthSession};
use chrono::{DateTime, Utc};
use derive_builder::WebApiGen;
use diesel::{AsChangeset, Insertable, Queryable, RunQueryDsl, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Clone,
    Serialize,
    Deserialize,
    Selectable,
    JsonSchema,
    Default,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(super::DbType))]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub group_id: i64,
    pub tenantry: String,
    pub remark: Option<String>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub is_delete: bool,
}
#[derive(
    Queryable,
    Clone,
    Serialize,
    Deserialize,
    Selectable,
    JsonSchema,
    Default,
    AsChangeset,
    Debug,
    WebApiGen,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(super::DbType))]
pub struct User {
    pub id: i64,
    /// # Username
    /// in eth mode it will be address
    pub username: String,
    /// # password
    /// in eth mode it will be signature
    pub password: String,
    pub group_id: i64,
    pub tenantry: String,
    pub remark: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub update_by: Option<i64>,
    pub is_delete: bool,
}

