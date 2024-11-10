pub mod group;
pub mod group_permission;
pub mod permission;
pub mod user;


#[cfg(feature = "postgres")]
pub  type DbType =diesel::pg::Pg;
