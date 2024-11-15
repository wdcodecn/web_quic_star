use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::env;
use web_quic_star::set_env;

fn main() {
    set_env();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = PostgresConnectionManager::new(database_url.parse().unwrap(), NoTls);
    let pool = r2d2::Pool::new(manager).unwrap();

    // for i in 0..10i32 {
    //     let pool = pool.clone();
    //     thread::spawn(move || {
    let mut client = pool.get().unwrap();
    client
        .execute("INSERT INTO groups_permissions  VALUES (9,9)", &[])
        .unwrap();
    //     });
    // }
}
