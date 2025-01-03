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
#[diesel(check_for_backend(super::DbType))]
pub struct NewPermission {
    pub name: String,
    pub remark: Option<String>,
    pub create_time: DateTime<Utc>,
    #[validate(range(min = 1))]
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
#[diesel(check_for_backend(super::DbType))]
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
