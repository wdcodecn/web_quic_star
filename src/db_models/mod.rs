pub mod group;
pub mod group_permission;
pub mod permission;
pub mod user;

#[cfg(feature = "postgres")]
pub type DbType = diesel::pg::Pg;
pub type ConnPool = r2d2::Pool<diesel::r2d2::ConnectionManager<Conn>>;
#[cfg(feature = "postgres")]
pub type Conn = diesel::PgConnection;
