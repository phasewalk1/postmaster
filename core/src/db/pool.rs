use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use lazy_static::lazy_static;
use std::env::var as env;

/// A Postgres connection pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod rocket {
    use super::env;
    use super::lazy_static;

    // A lazily initialized database url, needs to be set at runtime
    lazy_static! {
        pub static ref DATABASE_URL: String =
            env("DATABASE_URL").expect("DATABASE_URL must be set");
    }

    /// Initialize a connection pool
    pub fn init_pool() -> super::Pool {
        let mgr = super::ConnectionManager::<super::PgConnection>::new(&*DATABASE_URL);
        return super::Pool::builder()
            .max_size(10)
            .build(mgr)
            .expect("Failed to create pool.");
    }

    /// Raw pool state
    #[derive(Debug)]
    pub struct PoolState {
        state: diesel::r2d2::State,
    }

    impl PoolState {
        /// Read the underlying state
        pub fn read(&self) -> &diesel::r2d2::State {
            // unwrap the RC
            return &self.state;
        }
    }

    impl From<super::Pool> for PoolState {
        fn from(pool: super::Pool) -> Self {
            Self {
                state: pool.state(),
            }
        }
    }
}

pub mod tonic {
    use crate::db::PoolConn;

    super::lazy_static! {
        pub static ref TONIC_POOL: Pooler = {
           Pooler {
               raw_conn: super::Pool::builder()
                   .max_size(10)
                   .build(super::ConnectionManager::new(super::env("DATABASE_URL").expect("DATABASE_URL must be set")))
                   .expect("Failed to create TONIC_POOL"),
           }
        };
    }
    
    pub struct Pooler {
        pub raw_conn: super::Pool,
    }

    pub type TonicPoolInner = super::ConnectionManager<super::PgConnection>;

    impl Pooler {
        pub fn try_connect(&self) -> Result<PoolConn<TonicPoolInner>, tonic::Status> {
            let maybe_conn = self.raw_conn.get();
            match maybe_conn {
                Ok(conn) => return Ok(conn),
                Err(e) => return Err(tonic::Status::internal(format!("Error getting connection from Db pool")))
            }
        }
    }
}
