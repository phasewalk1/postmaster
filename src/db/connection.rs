use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub fn get_conn_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    return Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool.");
}

pub fn unary_connection() -> PgConnection {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&url).expect("Failed to connect to database.");
}
