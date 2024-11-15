use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use std::env;

pub mod group;
pub mod group_permission;
pub mod permission;
pub mod user;

#[cfg(feature = "postgres")]
pub type DbType = diesel::pg::Pg;

#[cfg(feature = "postgres")]
pub type ConnPool = Pool<ConnectionManager<Conn>>;

#[cfg(feature = "postgres")]
pub type Conn = PgConnection;

pub fn setup_connection_pool() -> ConnPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<Conn>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
