use diesel::{QueryDsl, SelectableHelper};
use std::env;
use web_quic_star::db_models::user::User;
use web_quic_star::db_models::{setup_connection_pool, LogicDeleteQuery, Paginate};
use web_quic_star::schema::users::dsl::users;
use web_quic_star::set_env;

fn main() {
    set_env();
    let connection_pool = setup_connection_pool();
    let mut pooled_connection = connection_pool.get().unwrap();
    let x = users
        .select(User::as_select())
        .logic_delete_query()
        .paginate(1, 10)
        .load_and_count_pages(&mut pooled_connection)
        .unwrap();

    println!("{:?}", x);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // let manager = PostgresConnectionManager::new(database_url.parse().unwrap(), NoTls);
    // let pool = r2d2::Pool::new(manager).unwrap();
    // let new = GroupsPermission { group_id: 0, permission_id: 0 };
    // let mut client = pool.get().unwrap();

    // diesel::insert_into(groups_permissions).values(new).execute(&mut client);
    // for i in 0..10i32 {
    //     let pool = pool.clone();
    //     thread::spawn(move || {
    // client
    //     .execute("INSERT INTO groups_permissions  VALUES (9,9)", &[])
    //     .unwrap();
    //     });
    // }
}
