pub mod group;
pub mod group_permission;
pub mod permission;
pub mod user;
pub mod user_with_group_views;
pub mod dex_gram;
pub mod dex_gram_new;

#[cfg(feature = "postgres")]
pub type DbType = diesel::pg::Pg;
pub type ConnPool = r2d2::Pool<diesel::r2d2::ConnectionManager<Conn>>;
#[cfg(feature = "postgres")]
pub type Conn = diesel::PgConnection;
