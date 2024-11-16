use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use std::env;
use diesel::{define_sql_function, dsl, BoxableExpression, QueryableByName};
use diesel::expression::AsExpression;

pub mod group;
pub mod group_permission;
pub mod permission;
pub mod user;
pub mod pagination;

#[cfg(feature = "postgres")]
pub type DbType = diesel::pg::Pg;

#[cfg(feature = "postgres")]
pub type ConnPool = Pool<ConnectionManager<Conn>>;

#[derive(QueryableByName)]
pub struct Count {
    #[sql_type = "diesel::sql_types::BigInt"]
    pub count: i64,
}

#[cfg(feature = "postgres")]
pub type Conn = diesel::PgConnection;

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
