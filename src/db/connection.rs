use diesel::prelude::*;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use lazy_static::lazy_static;

// A lazily initialized global pool of database connections.
// This is the main DB object that is passed around
lazy_static! {
    pub static ref DB_POOL: Pooler = {
        Pooler {
            raw_connection: PostgresPool::builder()
                .max_size(10)
                .build(ConnectionManager::new(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")))
                .expect("Failed to create pool."),
        }
    };
}

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
pub type PoolConn = PooledConnection<ConnectionManager<PgConnection>>;

pub struct Pooler {
    pub raw_connection: PostgresPool,
}

impl Pooler {
    pub fn try_connect(&self) -> Result<PoolConn, anyhow::Error> {
        let conn = self.raw_connection.get();
        match conn {
            Ok(connection) => return Ok(connection),
            Err(e) => return Err(anyhow::anyhow!("Failed to connect to database: {}", e)),
        }
    }
}


pub fn get_conn_pool() -> PostgresPool { 
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
