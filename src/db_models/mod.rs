use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use std::env;
use diesel::{QueryableByName};
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;
pub mod group;
pub mod group_permission;
pub mod permission;
pub mod user;

#[cfg(feature = "postgres")]
pub type DbType = diesel::pg::Pg;

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



/// example: let x = groups_permissions.select(GroupsPermission::as_select())
/// .paginate(3,Some(10)).load_and_count_pages(&mut connection)?;
pub trait PaginateAndFilterDelete: Sized {
    fn paginate(self, page_no: i64,page_size:Option<i64>) -> Paginated<Self>;
    const DEFAULT_PER_PAGE: i64 = 10;

}

impl<T> PaginateAndFilterDelete for T {
    fn paginate(self, page_no: i64,page_size:Option<i64>) -> Paginated<Self> {
        let page_size = page_size.unwrap_or(Self::DEFAULT_PER_PAGE);
        Paginated {
            query: self,
            per_page: page_size,
            page: page_no,
            offset: (page_no - 1) * page_size,
        }

    }
}


#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
    offset: i64,
}

impl<T> Paginated<T> {


    pub fn load_and_count_pages<'a, U>(self, conn: &mut PgConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<'a, PgConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.first().map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total_pages))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

#[cfg(feature = "postgres")]
impl<T> QueryFragment<DbType> for Paginated<T>
where
    T: QueryFragment<DbType>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DbType>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(1) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t where t.is_delete = false LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}
