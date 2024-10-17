// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use diesel::{Identifiable, Queryable, Selectable};
use serde::Deserialize;

#[derive(Queryable, Debug, Identifiable, Selectable)]
#[diesel(primary_key(group_id, permission_id))]
#[diesel(table_name = crate::schema::groups_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GroupsPermission {
    pub group_id: i64,
    pub permission_id: i64,
}
